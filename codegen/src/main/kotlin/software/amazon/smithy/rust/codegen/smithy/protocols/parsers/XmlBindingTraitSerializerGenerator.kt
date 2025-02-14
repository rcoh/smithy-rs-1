/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy.protocols.parsers

import software.amazon.smithy.codegen.core.CodegenException
import software.amazon.smithy.model.knowledge.HttpBinding
import software.amazon.smithy.model.knowledge.HttpBindingIndex
import software.amazon.smithy.model.shapes.BlobShape
import software.amazon.smithy.model.shapes.BooleanShape
import software.amazon.smithy.model.shapes.CollectionShape
import software.amazon.smithy.model.shapes.MapShape
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.NumberShape
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.Shape
import software.amazon.smithy.model.shapes.StringShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.shapes.TimestampShape
import software.amazon.smithy.model.shapes.UnionShape
import software.amazon.smithy.model.traits.EnumTrait
import software.amazon.smithy.model.traits.TimestampFormatTrait
import software.amazon.smithy.model.traits.XmlFlattenedTrait
import software.amazon.smithy.model.traits.XmlNamespaceTrait
import software.amazon.smithy.rust.codegen.rustlang.Attribute
import software.amazon.smithy.rust.codegen.rustlang.CargoDependency
import software.amazon.smithy.rust.codegen.rustlang.RustType
import software.amazon.smithy.rust.codegen.rustlang.RustWriter
import software.amazon.smithy.rust.codegen.rustlang.asType
import software.amazon.smithy.rust.codegen.rustlang.render
import software.amazon.smithy.rust.codegen.rustlang.rust
import software.amazon.smithy.rust.codegen.rustlang.rustBlock
import software.amazon.smithy.rust.codegen.rustlang.rustBlockTemplate
import software.amazon.smithy.rust.codegen.rustlang.rustTemplate
import software.amazon.smithy.rust.codegen.rustlang.stripOuter
import software.amazon.smithy.rust.codegen.rustlang.withBlock
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolConfig
import software.amazon.smithy.rust.codegen.smithy.isOptional
import software.amazon.smithy.rust.codegen.smithy.protocols.XmlMemberIndex
import software.amazon.smithy.rust.codegen.smithy.protocols.XmlNameIndex
import software.amazon.smithy.rust.codegen.smithy.rustType
import software.amazon.smithy.rust.codegen.util.dq
import software.amazon.smithy.rust.codegen.util.expectMember
import software.amazon.smithy.rust.codegen.util.getTrait
import software.amazon.smithy.rust.codegen.util.hasTrait
import software.amazon.smithy.rust.codegen.util.inputShape
import software.amazon.smithy.rust.codegen.util.toPascalCase
import software.amazon.smithy.rust.codegen.util.toSnakeCase

class XmlBindingTraitSerializerGenerator(protocolConfig: ProtocolConfig) : StructuredDataSerializerGenerator {
    private val symbolProvider = protocolConfig.symbolProvider
    private val runtimeConfig = protocolConfig.runtimeConfig
    private val model = protocolConfig.model
    private val smithyXml = CargoDependency.smithyXml(runtimeConfig).asType()
    private val codegenScope =
        arrayOf(
            "XmlWriter" to smithyXml.member("encode::XmlWriter"),
            "ElementWriter" to smithyXml.member("encode::ElWriter"),
            "SdkBody" to RuntimeType.sdkBody(runtimeConfig),
            // TODO: currently this doesn't ever actually fail, however, once unions have an unknown member
            // serialization can fail here and we should replace this with a real error type.
            // Currently the serialization errors just get converted into `OperationBuildError` by the code
            // that calls this code
            "Error" to RuntimeType("String", null, "std::string")
        )

    private val xmlIndex = XmlNameIndex.of(model)
    private val httpIndex = HttpBindingIndex.of(model)
    private val rootNamespace = protocolConfig.serviceShape.getTrait<XmlNamespaceTrait>()

    sealed class Ctx {
        abstract val input: String

        data class Element(val elementWriter: String, override val input: String) : Ctx()
        data class Scope(val scopeWriter: String, override val input: String) : Ctx()

        companion object {
            // Kotlin doesn't have a "This" type
            @Suppress("UNCHECKED_CAST")
            fun <T : Ctx> updateInput(input: T, newInput: String): T = when (input) {
                is Element -> input.copy(input = newInput) as T
                is Scope -> input.copy(input = newInput) as T
                else -> TODO()
            }
        }
    }

    private fun Ctx.Element.scopedTo(member: MemberShape) =
        this.copy(input = "$input.${symbolProvider.toMemberName(member)}")

    private fun Ctx.Scope.scopedTo(member: MemberShape) =
        this.copy(input = "$input.${symbolProvider.toMemberName(member)}")

    override fun operationSerializer(operationShape: OperationShape): RuntimeType? {
        val fnName = "serialize_operation_${operationShape.id.name.toSnakeCase()}"
        val inputShape = operationShape.inputShape(model)
        val xmlMembers = operationShape.operationXmlMembers()
        if (!xmlMembers.isNotEmpty()) {
            return null
        }
        val operationXmlName = xmlIndex.operationInputShapeName(operationShape)
            ?: throw CodegenException("operation must have a name if it has members")
        return RuntimeType.forInlineFun(fnName, "operation_ser") {
            it.rustBlockTemplate(
                "pub fn $fnName(input: &#{target}) -> Result<#{SdkBody}, #{Error}>",
                *codegenScope, "target" to symbolProvider.toSymbol(inputShape)
            ) {
                rust("let mut out = String::new();")
                // create a scope for writer. This ensure that writer has been dropped before returning the
                // string and ensures that all closing tags get written
                rustBlock("") {
                    rustTemplate(
                        """
                        let mut writer = #{XmlWriter}::new(&mut out);
                        ##[allow(unused_mut)]
                        let mut root = writer.start_el(${operationXmlName.dq()})${inputShape.xmlNamespace().apply()};
                    """,
                        *codegenScope
                    )
                    serializeStructure(inputShape, xmlMembers, Ctx.Element("root", "&input"))
                }
                rustTemplate("Ok(#{SdkBody}::from(out))", *codegenScope)
            }
        }
    }

    override fun documentSerializer(): RuntimeType {
        TODO("RestXML does not support document types")
    }

    override fun payloadSerializer(member: MemberShape): RuntimeType {
        val target = model.expectShape(member.target, StructureShape::class.java)
        val fnName = "serialize_payload_${target.id.name.toSnakeCase()}_${member.container.name.toSnakeCase()}"
        return RuntimeType.forInlineFun(fnName, "xml_ser") {
            val t = symbolProvider.toSymbol(member).rustType().stripOuter<RustType.Option>().render(true)
            it.rustBlock(
                "pub fn $fnName(input: &$t) -> Result<#T, String>",

                RuntimeType.sdkBody(runtimeConfig),
            ) {
                rust("let mut out = String::new();")
                // create a scope for writer. This ensure that writer has been dropped before returning the
                // string and ensures that all closing tags get written
                rustBlock("") {
                    rustTemplate(
                        """
                        let mut writer = #{XmlWriter}::new(&mut out);
                        ##[allow(unused_mut)]
                        let mut root = writer.start_el(${xmlIndex.payloadShapeName(member).dq()})${
                        target.xmlNamespace().apply()
                        };
                    """,
                        *codegenScope
                    )
                    serializeStructure(
                        target,
                        XmlMemberIndex.fromMembers(target.members().toList()),
                        Ctx.Element("root", "&input")
                    )
                }
                rustTemplate("Ok(#{SdkBody}::from(out))", *codegenScope)
            }
        }
    }

    private fun XmlNamespaceTrait?.apply(): String {
        this ?: return ""
        val prefix = prefix.map { prefix -> "Some(${prefix.dq()})" }.orElse("None")
        return ".write_ns(${uri.dq()}, $prefix)"
    }

    private fun RustWriter.structureInner(members: XmlMemberIndex, ctx: Ctx.Element) {
        if (members.attributeMembers.isNotEmpty()) {
            rust("let mut ${ctx.elementWriter} = ${ctx.elementWriter};")
        }
        members.attributeMembers.forEach { member ->
            handleOptional(member, ctx.scopedTo(member)) { ctx ->
                withBlock("${ctx.elementWriter}.write_attribute(${xmlIndex.memberName(member).dq()},", ");") {
                    serializeRawMember(member, ctx.input)
                }
            }
        }
        Attribute.AllowUnusedMut.render(this)
        rust("let mut scope = ${ctx.elementWriter}.finish();")
        val scopeCtx = Ctx.Scope("scope", ctx.input)
        members.dataMembers.forEach { member ->
            serializeMember(member, scopeCtx.scopedTo(member), null)
        }
        rust("scope.finish();")
    }

    /**
     * Dereference [input]
     *
     * Clippy is upset about `*&`, so if [input] is already referenced, simply strip the leading '&'
     */
    private fun autoDeref(input: String) = if (input.startsWith("&")) {
        input.removePrefix("&")
    } else {
        "*$input"
    }

    private fun RustWriter.serializeRawMember(member: MemberShape, input: String) {
        when (val shape = model.expectShape(member.target)) {
            is StringShape -> if (shape.hasTrait<EnumTrait>()) {
                rust("$input.as_str()")
            } else {
                rust("$input.as_ref()")
            }
            is NumberShape -> rust("$input.to_string().as_ref()")
            is BooleanShape -> rust("""if ${autoDeref(input)} { "true" } else { "false" }""")
            is BlobShape -> rust("#T($input.as_ref()).as_ref()", RuntimeType.Base64Encode(runtimeConfig))
            is TimestampShape -> {
                val timestampFormat =
                    httpIndex.determineTimestampFormat(
                        member,
                        HttpBinding.Location.DOCUMENT,
                        TimestampFormatTrait.Format.DATE_TIME
                    )
                val timestampFormatType = RuntimeType.TimestampFormat(runtimeConfig, timestampFormat)
                rust("$input.fmt(#T).as_ref()", timestampFormatType)
            }
            else -> TODO(member.toString())
        }
    }

    @Suppress("NAME_SHADOWING")
    private fun RustWriter.serializeMember(memberShape: MemberShape, ctx: Ctx.Scope, rootNameOverride: String? = null) {
        val target = model.expectShape(memberShape.target)
        val xmlName = rootNameOverride ?: xmlIndex.memberName(memberShape)
        val ns = memberShape.xmlNamespace().apply()
        handleOptional(memberShape, ctx) { ctx ->
            when (target) {
                is StringShape, is BooleanShape, is NumberShape, is TimestampShape, is BlobShape -> {
                    rust("let mut inner_writer = ${ctx.scopeWriter}.start_el(${xmlName.dq()})$ns.finish();")
                    withBlock("inner_writer.data(", ");") {
                        serializeRawMember(memberShape, ctx.input)
                    }
                }
                is CollectionShape -> if (memberShape.hasTrait<XmlFlattenedTrait>()) {
                    serializeFlatList(memberShape, target, ctx)
                } else {
                    rust("let mut inner_writer = ${ctx.scopeWriter}.start_el(${xmlName.dq()})$ns.finish();")
                    serializeList(target, Ctx.Scope("inner_writer", ctx.input))
                }
                is MapShape -> if (memberShape.hasTrait<XmlFlattenedTrait>()) {
                    serializeMap(target, xmlIndex.memberName(memberShape), ctx)
                } else {
                    rust("let mut inner_writer = ${ctx.scopeWriter}.start_el(${xmlName.dq()})$ns.finish();")
                    serializeMap(target, "entry", Ctx.Scope("inner_writer", ctx.input))
                }
                is StructureShape -> {
                    rust("let inner_writer = ${ctx.scopeWriter}.start_el(${xmlName.dq()})$ns;")
                    serializeStructure(
                        target,
                        XmlMemberIndex.fromMembers(target.members().toList()),
                        Ctx.Element("inner_writer", ctx.input)
                    )
                }
                is UnionShape -> {
                    rust("let inner_writer = ${ctx.scopeWriter}.start_el(${xmlName.dq()})$ns;")
                    serializeUnion(target, Ctx.Element("inner_writer", ctx.input))
                }
                else -> TODO(target.toString())
            }
        }
    }

    private fun RustWriter.serializeStructure(
        structureShape: StructureShape,
        members: XmlMemberIndex,
        ctx: Ctx.Element
    ) {
        val fnName = "serialize_structure_${structureShape.id.name.toSnakeCase()}"
        val structureSymbol = symbolProvider.toSymbol(structureShape)
        val structureSerializer = RuntimeType.forInlineFun(fnName, "xml_ser") {
            it.rustBlockTemplate(
                "pub fn $fnName(input: &#{Shape}, writer: #{ElementWriter})",
                "Shape" to structureSymbol,
                *codegenScope
            ) {
                if (!members.isNotEmpty()) {
                    // removed unused warning if there are no fields we're going to read
                    rust("let _ = input;")
                }
                structureInner(members, Ctx.Element("writer", "&input"))
            }
        }
        rust("#T(&${ctx.input}, ${ctx.elementWriter})", structureSerializer)
    }

    private fun RustWriter.serializeUnion(unionShape: UnionShape, ctx: Ctx.Element) {
        val fnName = "serialize_union_${unionShape.id.name.toSnakeCase()}"
        val unionSymbol = symbolProvider.toSymbol(unionShape)
        val structureSerializer = RuntimeType.forInlineFun(fnName, "xml_ser") {
            it.rustBlockTemplate(
                "pub fn $fnName(input: &#{Shape}, writer: #{ElementWriter})",
                "Shape" to unionSymbol,
                *codegenScope
            ) {
                rust("let mut scope_writer = writer.finish();")
                rustBlock("match input") {
                    val members = unionShape.members()
                    members.forEach { member ->
                        val variantName = member.memberName.toPascalCase()
                        withBlock("#T::$variantName(inner) =>", ",", unionSymbol) {
                            serializeMember(member, Ctx.Scope("scope_writer", "inner"))
                        }
                    }
                }
            }
        }
        rust("#T(&${ctx.input}, ${ctx.elementWriter})", structureSerializer)
    }

    private fun RustWriter.serializeList(listShape: CollectionShape, ctx: Ctx.Scope) {
        val itemName = safeName("list_item")
        rustBlock("for $itemName in ${ctx.input}") {
            serializeMember(listShape.member, ctx.copy(input = itemName))
        }
    }

    private fun RustWriter.serializeFlatList(member: MemberShape, listShape: CollectionShape, ctx: Ctx.Scope) {
        val itemName = safeName("list_item")
        rustBlock("for $itemName in ${ctx.input}") {
            serializeMember(listShape.member, ctx.copy(input = itemName), xmlIndex.memberName(member))
        }
    }

    private fun RustWriter.serializeMap(mapShape: MapShape, entryName: String, ctx: Ctx.Scope) {
        val key = safeName("key")
        val value = safeName("value")
        rustBlock("for ($key, $value) in ${ctx.input}") {
            rust("""let mut entry = ${ctx.scopeWriter}.start_el(${entryName.dq()}).finish();""")
            serializeMember(mapShape.key, ctx.copy(scopeWriter = "entry", input = key))
            serializeMember(mapShape.value, ctx.copy(scopeWriter = "entry", input = value))
        }
    }

    /**
     * If [member] is an optional shape, generate code like:
     * ```rust
     * if let Some(inner) = member {
     *   // .. BLOCK
     * }
     * ```
     *
     * If [member] is not an optional shape, generate code like:
     * `{ .. Block }`
     *
     * [inner] is passed a new `ctx` object to use for code generation which handles the
     * potentially new name of the input.
     */
    private fun <T : Ctx> RustWriter.handleOptional(member: MemberShape, ctx: T, inner: RustWriter.(T) -> Unit) {
        val memberSymbol = symbolProvider.toSymbol(member)
        if (memberSymbol.isOptional()) {
            val tmp = safeName()
            rustBlock("if let Some($tmp) = ${ctx.input}") {
                inner(Ctx.updateInput(ctx, tmp))
            }
        } else {
            rustBlock("") {
                inner(ctx)
            }
        }
    }

    private fun OperationShape.operationXmlMembers(): XmlMemberIndex {
        val outputShape = this.inputShape(model)
        val documentMembers =
            httpIndex.getRequestBindings(this).filter { it.value.location == HttpBinding.Location.DOCUMENT }
                .keys.map { outputShape.expectMember(it) }
        return XmlMemberIndex.fromMembers(documentMembers)
    }

    private fun Shape.xmlNamespace(): XmlNamespaceTrait? {
        return this.getTrait() ?: rootNamespace
    }
}

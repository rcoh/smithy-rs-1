/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy.protocols

import software.amazon.smithy.model.knowledge.HttpBinding
import software.amazon.smithy.model.knowledge.HttpBindingIndex
import software.amazon.smithy.model.shapes.BlobShape
import software.amazon.smithy.model.shapes.BooleanShape
import software.amazon.smithy.model.shapes.CollectionShape
import software.amazon.smithy.model.shapes.MapShape
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.NumberShape
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.StringShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.shapes.TimestampShape
import software.amazon.smithy.model.shapes.UnionShape
import software.amazon.smithy.model.traits.EnumTrait
import software.amazon.smithy.model.traits.TimestampFormatTrait
import software.amazon.smithy.model.traits.XmlAttributeTrait
import software.amazon.smithy.model.traits.XmlFlattenedTrait
import software.amazon.smithy.model.traits.XmlNameTrait
import software.amazon.smithy.rust.codegen.rustlang.CargoDependency
import software.amazon.smithy.rust.codegen.rustlang.RustType
import software.amazon.smithy.rust.codegen.rustlang.RustWriter
import software.amazon.smithy.rust.codegen.rustlang.asType
import software.amazon.smithy.rust.codegen.rustlang.conditionalBlock
import software.amazon.smithy.rust.codegen.rustlang.escape
import software.amazon.smithy.rust.codegen.rustlang.rust
import software.amazon.smithy.rust.codegen.rustlang.rustBlock
import software.amazon.smithy.rust.codegen.rustlang.rustBlockTemplate
import software.amazon.smithy.rust.codegen.rustlang.rustTemplate
import software.amazon.smithy.rust.codegen.rustlang.withBlock
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolConfig
import software.amazon.smithy.rust.codegen.smithy.generators.StructureGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.builderSymbol
import software.amazon.smithy.rust.codegen.smithy.generators.setterName
import software.amazon.smithy.rust.codegen.smithy.isBoxed
import software.amazon.smithy.rust.codegen.smithy.isOptional
import software.amazon.smithy.rust.codegen.smithy.traits.SyntheticOutputTrait
import software.amazon.smithy.rust.codegen.util.dq
import software.amazon.smithy.rust.codegen.util.expectMember
import software.amazon.smithy.rust.codegen.util.orNull
import software.amazon.smithy.rust.codegen.util.outputShape
import software.amazon.smithy.rust.codegen.util.toPascalCase
import software.amazon.smithy.rust.codegen.util.toSnakeCase

class RestXmlParserGenerator(protocolConfig: ProtocolConfig) {

    /**
     * Abstraction to represent an XML element name:
     * `[prefix]:[local]`
     */
    data class XmlName(val local: String, val prefix: String? = null) {
        override fun toString(): String {
            return prefix?.let { "$it:" }.orEmpty() + local
        }

        companion object {
            fun parse(v: String): XmlName {
                val split = v.indexOf(':')
                return if (split == -1) {
                    XmlName(local = v, prefix = null)
                } else {
                    XmlName(v.substring(split + 1), prefix = v.substring(0, split))
                }
            }
        }
    }

    /**
     * Codegeneration Context
     *
     * [tag]: The symbol name of the current tag
     * [currentTarget]: An expression that represents the symbol currently being written to. Required to support flattened
     * structures.
     */
    data class Ctx(val tag: String, val currentTarget: String?)

    private val symbolProvider = protocolConfig.symbolProvider
    private val smithyXml = CargoDependency.smithyXml(protocolConfig.runtimeConfig).asType()
    private val xmlError = smithyXml.member("decode::XmlError")

    private val scopedDecoder = smithyXml.member("decode::ScopedDecoder")
    private val runtimeConfig = protocolConfig.runtimeConfig
    // The symbols we want all the time
    private val codegenScope = arrayOf(
        "Blob" to RuntimeType.Blob(runtimeConfig),
        "Document" to smithyXml.member("decode::Document"),
        "XmlError" to xmlError,
        "next_start_element" to smithyXml.member("decode::next_start_element"),
        "expect_data" to smithyXml.member("decode::expect_data"),
        "ScopedDecoder" to scopedDecoder
    )
    private val model = protocolConfig.model
    private val index = HttpBindingIndex.of(model)

    /**
     * Generate a parse function for a given targeted as a payload.
     * Entry point for payload-based parsing.
     * Roughly:
     * ```rust
     * fn parse_my_struct(input: &[u8]) -> Result<MyStruct, XmlError> {
     *      ...
     * }
     * ```
     */
    fun payloadParser(member: MemberShape): RuntimeType {
        val shape = model.expectShape(member.target)
        check(shape is UnionShape || shape is StructureShape) { "structure parser should only be used on structures & unions" }
        val fnName = shape.id.name.toString().toSnakeCase()
        return RuntimeType.forInlineFun(fnName, "xml_deser") {
            it.rustBlock(
                "pub fn $fnName(inp: &[u8]) -> Result<#1T, #2T>",
                symbolProvider.toSymbol(shape),
                xmlError
            ) {
                val shapeName =
                    member.getMemberTrait(model, XmlNameTrait::class.java).orNull()?.let { XmlName.parse(it.value) }
                        ?: XmlName(local = shape.id.name)
                rustTemplate(
                    """
                    use std::convert::TryFrom;
                    let mut doc = #{Document}::try_from(inp)?;
                    let mut decoder = doc.root_element()?;
                    let start_el = decoder.start_el();
                    if !(${shapeName.compareTo("start_el")}) {
                        return Err(#{XmlError}::Custom(format!("invalid root, expected $shapeName got {:?}", start_el)))
                    }
                    """,
                    *codegenScope
                )
                val ctx = Ctx("decoder", currentTarget = null)
                withBlock("Ok(", ")") {
                    when (shape) {
                        is StructureShape -> parseStructure(shape, ctx)
                        is UnionShape -> parseUnion(shape, ctx)
                    }
                }
            }
        }
    }

    /** Generate a parser for operation input
     * Because only a subset of fields of the operation may be impacted by the document, a builder is passed
     * through:
     *
     * ```rust
     * fn parse_some_operation(inp: &[u8], builder: my_operation::Builder) -> Result<my_operation::Builder, XmlError> {
     *   ...
     * }
     * ```
     */
    fun operationParser(operationShape: OperationShape): RuntimeType {
        val outputShape = operationShape.outputShape(model)
        val fnName = outputShape.id.name.toString().toSnakeCase()
        return RuntimeType.forInlineFun(fnName, "xml_deser") {
            it.rustBlock(
                "pub fn $fnName(inp: &[u8], mut builder: #1T) -> Result<#1T, #2T>",
                outputShape.builderSymbol(symbolProvider),
                xmlError
            ) {
                val shapeName = XmlName(
                    local = outputShape.expectTrait(SyntheticOutputTrait::class.java).originalId!!.name,
                    prefix = null
                )
                rustTemplate(
                    """
                    use std::convert::TryFrom;
                    let mut doc = #{Document}::try_from(inp)?;
                    let mut decoder = doc.root_element()?;
                    let start_el = decoder.start_el();
                    if !(${shapeName.compareTo("start_el")}) {
                        return Err(#{XmlError}::Custom(format!("invalid root, expected $shapeName got {:?}", start_el)))
                    }
                    """,
                    *codegenScope
                )
                val members = operationShape.operationXmlMembers()
                parseStructureInner(members, builder = "builder", Ctx(tag = "decoder", currentTarget = null))
                rust("Ok(builder)")
            }
        }
    }

    /**
     * Remove duplicated code on the operation/structure path
     */
    private fun RustWriter.parseStructureInner(members: XmlMemberIndex, builder: String, outerCtx: Ctx) {
        members.attributeMembers.forEach { member ->
            val temp = safeName("attrib")
            withBlock("let $temp = ", ";") {
                parseAttributeMember(member, outerCtx)
            }
            rust("$builder.${symbolProvider.toMemberName(member)} = $temp;")
        }
        parseLoop(outerCtx) { ctx ->
            members.dataMembers.forEach { member ->
                case(member) {
                    val temp = safeName()
                    withBlock("let $temp = ", ";") {
                        parseMember(
                            member,
                            ctx.copy(currentTarget = "$builder.${symbolProvider.toMemberName(member)}.take()")
                        )
                    }
                    rust("$builder = $builder.${member.setterName()}($temp);")
                }
            }
        }
    }

    /**
     * The core XML parsing abstraction: A loop that reads through the top level tags at the current scope &
     * generates a match expression
     * When [ignoreUnexpected] is true, unexpected tags are ignored
     */
    private fun RustWriter.parseLoop(ctx: Ctx, ignoreUnexpected: Boolean = true, inner: RustWriter.(Ctx) -> Unit) {
        rustBlock("while let Some(mut tag) = ${ctx.tag}.next_tag()") {
            rustBlock("match tag.start_el()") {
                inner(ctx.copy(tag = "tag"))
                if (ignoreUnexpected) {
                    rust("_ => {}")
                }
            }
        }
    }

    /**
     * Generate an XML parser for a given member
     */
    private fun RustWriter.parseMember(memberShape: MemberShape, ctx: Ctx) {
        val target = model.expectShape(memberShape.target)
        val symbol = symbolProvider.toSymbol(memberShape)
        conditionalBlock("Some(", ")", symbol.isOptional()) {
            conditionalBlock("Box::new(", ")", symbol.isBoxed()) {
                when (target) {
                    is StringShape, is BooleanShape, is NumberShape, is TimestampShape, is BlobShape -> parsePrimitiveInner(
                        memberShape
                    ) {
                        rustTemplate("#{expect_data}(&mut ${ctx.tag})?", *codegenScope)
                    }
                    is MapShape -> if (memberShape.isFlattened()) {
                        parseFlatMap(target, ctx)
                    } else {
                        parseMap(target, ctx)
                    }
                    is CollectionShape -> if (memberShape.isFlattened()) {
                        parseFlatList(target, ctx)
                    } else {
                        parseList(target, ctx)
                    }
                    is StructureShape -> parseStructure(target, ctx)
                    is UnionShape -> parseUnion(target, ctx)
                    else -> rust("todo!(${escape(target.toString()).dq()})")
                }
            }
        }
    }

    private fun RustWriter.parseAttributeMember(memberShape: MemberShape, ctx: Ctx) {
        rustBlock("") {
            rustTemplate(
                """let s = ${ctx.tag}
                    .start_el()
                    .attr(${memberShape.xmlName().toString().dq()});""",
                *codegenScope
            )
            rustBlock("match s") {
                rust("None => None,")
                withBlock("Some(s) => Some(", ")") {
                    parsePrimitiveInner(memberShape) {
                        rust("s")
                    }
                }
            }
        }
    }

    private fun RustWriter.parseUnion(shape: UnionShape, ctx: Ctx) {
        val fnName = shape.id.name.toString().toSnakeCase() + "_inner"
        val symbol = symbolProvider.toSymbol(shape)
        val nestedParser = RuntimeType.forInlineFun(fnName, "xml_deser") {
            it.rustBlockTemplate(
                "pub fn $fnName(decoder: &mut #{ScopedDecoder}) -> Result<#{Shape}, #{XmlError}>",
                *codegenScope, "Shape" to symbol
            ) {
                val members = shape.members()
                parseLoop(Ctx(tag = "decoder", currentTarget = null)) { ctx ->
                    members.forEach { member ->
                        case(member) {
                            withBlock("return Ok(#T::${member.memberName.toPascalCase()}(", "))", symbol) {
                                parseMember(member, ctx)
                            }
                        }
                    }
                }
                rustTemplate("""Err(#{XmlError}::Other { msg: "expected union, got nothing..."})""", *codegenScope)
            }
        }
        rust("#T(&mut ${ctx.tag})?", nestedParser)
    }

    /**
     * The match clause to check if the tag matches a given member
     */
    private fun RustWriter.case(member: MemberShape, inner: RustWriter.() -> Unit) {
        rustBlock("s if ${member.xmlName().compareTo("s")} => ") {
            inner()
        }
        rust(",")
    }

    private fun RustWriter.parseStructure(shape: StructureShape, ctx: Ctx) {
        val fnName = shape.id.name.toString().toSnakeCase() + "_inner"
        val symbol = symbolProvider.toSymbol(shape)
        val nestedParser = RuntimeType.forInlineFun(fnName, "xml_deser") {
            it.rustBlockTemplate(
                "pub fn $fnName(decoder: &mut #{ScopedDecoder}) -> Result<#{Shape}, #{XmlError}>",
                *codegenScope, "Shape" to symbol
            ) {
                rustTemplate(
                    """
                    let mut builder = #{Shape}::builder();
                """,
                    *codegenScope, "Shape" to symbol
                )
                val members = shape.xmlMembers()
                parseStructureInner(members, "builder", Ctx(tag = "decoder", currentTarget = null))
                withBlock("Ok(builder.build()", ")") {
                    if (StructureGenerator.fallibleBuilder(shape, symbolProvider)) {
                        rust(""".map_err(|_|{XmlError}::Other { msg: "missing field"})?""")
                    }
                }
            }
        }
        rust("#T(&mut ${ctx.tag})?", nestedParser)
    }

    private fun RustWriter.parseList(target: CollectionShape, ctx: Ctx) {
        val fnName = "deserialize_${target.member.id.name.toSnakeCase()}"
        val member = target.member
        val listParser = RuntimeType.forInlineFun(fnName, "xml_deser") {
            it.rustBlockTemplate(
                "pub fn $fnName(decoder: &mut #{ScopedDecoder}) -> Result<#{List}, #{XmlError}>",
                *codegenScope,
                "List" to symbolProvider.toSymbol(target)
            ) {
                rust("let mut out = std::vec::Vec::new();")
                parseLoop(Ctx(tag = "decoder", currentTarget = null)) { ctx ->
                    case(member) {
                        withBlock("out.push(", ");") {
                            parseMember(member, ctx)
                        }
                    }
                }
                rust("Ok(out)")
            }
        }
        rust("#T(&mut ${ctx.tag})?", listParser)
    }

    private fun RustWriter.parseFlatList(target: CollectionShape, ctx: Ctx) {
        val list = safeName("list")
        rustBlock("") {
            rust("let mut $list = ${ctx.currentTarget!!}.unwrap_or_default();")
            withBlock("$list.push(", ");") {
                parseMember(target.member, ctx)
            }
            rust(list)
        }
    }

    private fun RustWriter.parseMap(target: MapShape, ctx: Ctx) {
        val fnName = "deserialize_${target.value.id.name.toSnakeCase()}"
        val mapParser = RuntimeType.forInlineFun(fnName, "xml_deser") {
            it.rustBlockTemplate(
                "pub fn $fnName(decoder: &mut #{ScopedDecoder}) -> Result<#{Map}, #{XmlError}>",
                *codegenScope,
                "Map" to symbolProvider.toSymbol(target)
            ) {
                rust("let mut out = #T::new();", RustType.HashMap.RuntimeType)
                parseLoop(Ctx(tag = "decoder", currentTarget = null)) { ctx ->
                    rustBlock("s if ${XmlName(local = "entry").compareTo("s")} => ") {
                        rust("#T(&mut ${ctx.tag}, &mut out)?;", mapEntryParser(target, ctx))
                    }
                }
                rust("Ok(out)")
            }
        }
        rust("#T(&mut ${ctx.tag})?", mapParser)
    }

    private fun RustWriter.parseFlatMap(target: MapShape, ctx: Ctx) {
        val map = safeName("map")
        val entryDecoder = mapEntryParser(target, ctx)
        rust(
            """{
            let mut $map = ${ctx.currentTarget!!}.unwrap_or_default();
            #T(&mut tag, &mut $map)?;
            $map
            }
            """,
            entryDecoder
        )
    }

    private fun mapEntryParser(
        target: MapShape,
        ctx: Ctx
    ): RuntimeType {

        val fnName = target.value.id.name.toSnakeCase() + "_entry"
        return RuntimeType.forInlineFun(fnName, "xml_deser") {
            it.rustBlockTemplate(
                "pub fn $fnName(decoder: &mut #{ScopedDecoder}, out: &mut #{Map}) -> Result<(), #{XmlError}>",
                *codegenScope,
                "Map" to symbolProvider.toSymbol(target)
            ) {
                rust("let mut k: Option<String> = None;")
                rust("let mut v: Option<#T> = None;", symbolProvider.toSymbol(model.expectShape(target.value.target)))
                parseLoop(Ctx("decoder", currentTarget = null)) {
                    case(target.key) {
                        withBlock("k = Some(", ")") {
                            parseMember(target.key, ctx = ctx.copy(currentTarget = null))
                        }
                    }
                    case(target.value) {
                        withBlock("v = Some(", ")") {
                            parseMember(target.value, ctx = ctx.copy(currentTarget = "v"))
                        }
                    }
                }

                rustTemplate(
                    """
                let k = k.ok_or(#{XmlError}::Other { msg: "missing key map entry"})?;
                let v = v.ok_or(#{XmlError}::Other { msg: "missing value map entry"})?;
                out.insert(k, v);
                Ok(())
                        """,
                    *codegenScope
                )
            }
        }
    }

    /**
     * Parse a simple member from a data field
     * [provider] generates code for the inner data field
     */
    private fun RustWriter.parsePrimitiveInner(member: MemberShape, provider: RustWriter.() -> Unit) {
        when (val shape = model.expectShape(member.target)) {
            is StringShape -> parseStringInner(shape, provider)
            is NumberShape, is BooleanShape -> {
                rustBlock("") {
                    rust("use std::str::FromStr;")
                    withBlock("#T::from_str(", ")", symbolProvider.toSymbol(shape)) {
                        provider()
                    }
                    rustTemplate(
                        """.map_err(|_|#{XmlError}::Other { msg: "expected ${escape(shape.toString())}"})?""",
                        *codegenScope
                    )
                }
            }
            is TimestampShape -> {
                val timestampFormat =
                    index.determineTimestampFormat(
                        member,
                        HttpBinding.Location.DOCUMENT,
                        TimestampFormatTrait.Format.DATE_TIME
                    )
                val timestampFormatType = RuntimeType.TimestampFormat(runtimeConfig, timestampFormat)
                withBlock("#T::from_str(", ")", RuntimeType.Instant(runtimeConfig)) {
                    provider()
                    rust(", #T", timestampFormatType)
                }
                rustTemplate(
                    """.map_err(|_|#{XmlError}::Other { msg: "expected ${escape(shape.toString())}"})?""",
                    *codegenScope
                )
            }
            is BlobShape -> {
                withBlock("#T(", ")", RuntimeType.Base64Decode(runtimeConfig)) {
                    provider()
                }
                rustTemplate(
                    """.map_err(|_|#{XmlError}::Other { msg: "invalid base64"}).map(#{Blob}::new)?""",
                    *codegenScope
                )
            }
            else -> TODO(shape.toString())
        }
    }

    private fun RustWriter.parseStringInner(shape: StringShape, provider: RustWriter.() -> Unit) {
        val enumTrait = shape.getTrait(EnumTrait::class.java).orElse(null)
        if (enumTrait == null) {
            provider()
            rust(".to_string()")
        } else {
            val enumSymbol = symbolProvider.toSymbol(shape)
            withBlock("#T::from(", ")", enumSymbol) {
                provider()
            }
        }
    }

    private fun MemberShape.xmlName(): XmlName {
        val override = this.getMemberTrait(model, XmlNameTrait::class.java).orNull()
        return override?.let { XmlName.parse(it.value) } ?: XmlName(local = this.memberName)
    }

    private fun MemberShape.isFlattened(): Boolean {
        return getMemberTrait(model, XmlFlattenedTrait::class.java).isPresent
    }

    fun XmlName.compareTo(start_el: String) =
        "$start_el.matches(${this.toString().dq()})"

    data class XmlMemberIndex(val dataMembers: List<MemberShape>, val attributeMembers: List<MemberShape>) {
        companion object {
            fun fromMembers(members: List<MemberShape>): XmlMemberIndex {
                val (attribute, data) = members.partition { it.hasTrait(XmlAttributeTrait::class.java) }
                return XmlMemberIndex(data, attribute)
            }
        }
    }

    private fun OperationShape.operationXmlMembers(): XmlMemberIndex {
        val outputShape = this.outputShape(model)
        val documentMembers =
            index.getResponseBindings(this).filter { it.value.location == HttpBinding.Location.DOCUMENT }
                .keys.map { outputShape.expectMember(it) }
        return XmlMemberIndex.fromMembers(documentMembers)
    }

    private fun StructureShape.xmlMembers(): XmlMemberIndex {
        return XmlMemberIndex.fromMembers(this.members().toList())
    }
}
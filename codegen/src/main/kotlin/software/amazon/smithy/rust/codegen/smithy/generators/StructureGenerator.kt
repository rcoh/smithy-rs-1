/*
 * Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *  http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 *
 *
 */

package software.amazon.smithy.rust.codegen.smithy.generators

import software.amazon.smithy.codegen.core.SymbolProvider
import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.traits.ErrorTrait
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.utils.CaseUtils

// TODO: extract struct generation from Smithy shapes to support generating body objects
class StructureGenerator(
    val model: Model,
    private val symbolProvider: SymbolProvider,
    private val writer: RustWriter,
    private val shape: StructureShape
) {
    private val sortedMembers: List<MemberShape> = shape.allMembers.values.sortedBy { symbolProvider.toMemberName(it) }
    fun render() {
        renderStructure()
        val errorTrait = shape.getTrait(ErrorTrait::class.java)
        errorTrait.map {
            val errorGenerator = ErrorGenerator(model, symbolProvider, writer, shape, it)
            errorGenerator.render()
        }
    }

    private fun renderStructure() {
        val symbol = symbolProvider.toSymbol(shape)
        // TODO: _probably_, pull this info from the symbol so that the
        // symbol provider can alter things as necessary
        writer.write("#[non_exhaustive]")
        writer.write("#[derive(Debug, PartialEq, Clone)]")
        val blockWriter = writer.openBlock("pub struct ${symbol.name} {")
        sortedMembers.forEach { member ->
            val memberName = member.memberName.toSnakeCase()
            blockWriter.write("pub $memberName: \$T,", symbolProvider.toSymbol(member)) }
        blockWriter.closeBlock("}")
    }
}

// String extensions
fun String.toSnakeCase(): String {
    return CaseUtils.toSnakeCase(this)
}

fun String.toPascalCase(): String {
    return CaseUtils.toSnakeCase(this).let { CaseUtils.toPascalCase(it) }
}

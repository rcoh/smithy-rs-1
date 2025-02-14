/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rustsdk

import software.amazon.smithy.aws.traits.auth.SigV4Trait
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.ServiceShape
import software.amazon.smithy.model.shapes.ShapeId
import software.amazon.smithy.rust.codegen.rustlang.Writable
import software.amazon.smithy.rust.codegen.rustlang.asType
import software.amazon.smithy.rust.codegen.rustlang.rust
import software.amazon.smithy.rust.codegen.rustlang.rustTemplate
import software.amazon.smithy.rust.codegen.rustlang.writable
import software.amazon.smithy.rust.codegen.smithy.RuntimeConfig
import software.amazon.smithy.rust.codegen.smithy.customize.OperationCustomization
import software.amazon.smithy.rust.codegen.smithy.customize.OperationSection
import software.amazon.smithy.rust.codegen.smithy.customize.RustCodegenDecorator
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolConfig
import software.amazon.smithy.rust.codegen.smithy.generators.config.ConfigCustomization
import software.amazon.smithy.rust.codegen.smithy.generators.config.ServiceConfig
import software.amazon.smithy.rust.codegen.smithy.letIf
import software.amazon.smithy.rust.codegen.util.dq
import software.amazon.smithy.rust.codegen.util.expectTrait
import software.amazon.smithy.rust.codegen.util.hasTrait

/**
 * The SigV4SigningDecorator:
 * - adds a `signing_service()` method to `config` to return the default signing service
 * - sets the `SigningService` during operation construction
 * - sets a default `OperationSigningConfig` A future enhancement will customize this for specific services that need
 *   different behavior.
 */
class SigV4SigningDecorator : RustCodegenDecorator {
    override val name: String = "SigV4Signing"
    override val order: Byte = 0

    private fun applies(protocolConfig: ProtocolConfig): Boolean = protocolConfig.serviceShape.hasTrait<SigV4Trait>()

    override fun configCustomizations(
        protocolConfig: ProtocolConfig,
        baseCustomizations: List<ConfigCustomization>
    ): List<ConfigCustomization> {
        return baseCustomizations.letIf(applies(protocolConfig)) {
            it + SigV4SigningConfig(protocolConfig.serviceShape.expectTrait())
        }
    }

    override fun operationCustomizations(
        protocolConfig: ProtocolConfig,
        operation: OperationShape,
        baseCustomizations: List<OperationCustomization>
    ): List<OperationCustomization> {
        return baseCustomizations.letIf(applies(protocolConfig)) {
            it + SigV4SigningFeature(protocolConfig.runtimeConfig, protocolConfig.serviceShape)
        }
    }
}

class SigV4SigningConfig(private val sigV4Trait: SigV4Trait) : ConfigCustomization() {
    override fun section(section: ServiceConfig): Writable {
        return when (section) {
            is ServiceConfig.ConfigImpl -> writable {
                rust(
                    """
                    /// The signature version 4 service signing name to use in the credential scope when signing requests.
                    ///
                    /// The signing service may be overidden by the `Endpoint`, or by specifying a custom [`SigningService`](aws_types::SigningService) during
                    /// operation construction
                    pub fn signing_service(&self) -> &'static str {
                        ${sigV4Trait.name.dq()}
                    }
                    """
                )
            }
            else -> emptySection
        }
    }
}

fun needsAmzSha256(service: ServiceShape) = when {
    service.id == ShapeId.from("com.amazonaws.s3#AmazonS3") -> true
    else -> false
}

fun disableDoubleEncode(service: ServiceShape) = when {
    service.id == ShapeId.from("com.amazonaws.s3#AmazonS3") -> true
    else -> false
}

class SigV4SigningFeature(private val runtimeConfig: RuntimeConfig, private val service: ServiceShape) :
    OperationCustomization() {
    override fun section(section: OperationSection): Writable {
        return when (section) {
            is OperationSection.MutateRequest -> writable {
                // TODO: this needs to be customized for individual operations, not just `default_config()`
                rustTemplate(
                    """
                ##[allow(unused_mut)]
                let mut signing_config = #{sig_auth}::signer::OperationSigningConfig::default_config();
                """,
                    "sig_auth" to runtimeConfig.sigAuth().asType()
                )
                if (needsAmzSha256(service)) {
                    rust("signing_config.signing_options.content_sha256_header = true;")
                }
                if (disableDoubleEncode(service)) {
                    rust("signing_config.signing_options.double_uri_encode = false;")
                }
                rustTemplate(
                    """
                ${section.request}.config_mut().insert(signing_config);
                ${section.request}.config_mut().insert(#{aws_types}::SigningService::from_static(${section.config}.signing_service()));
                """,
                    "aws_types" to awsTypes(runtimeConfig).asType()
                )
            }
            else -> emptySection
        }
    }
}

fun RuntimeConfig.sigAuth() = awsRuntimeDependency("aws-sig-auth")

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_batch_execute_statement_input_body(
    input: &crate::input::BatchExecuteStatementInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::BatchExecuteStatementInputBody {
        statements: &input.statements,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_batch_get_item_input_body(
    input: &crate::input::BatchGetItemInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::BatchGetItemInputBody {
        request_items: &input.request_items,
        return_consumed_capacity: &input.return_consumed_capacity,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_batch_write_item_input_body(
    input: &crate::input::BatchWriteItemInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::BatchWriteItemInputBody {
        request_items: &input.request_items,
        return_consumed_capacity: &input.return_consumed_capacity,
        return_item_collection_metrics: &input.return_item_collection_metrics,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_create_backup_input_body(
    input: &crate::input::CreateBackupInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::CreateBackupInputBody {
        table_name: &input.table_name,
        backup_name: &input.backup_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_create_global_table_input_body(
    input: &crate::input::CreateGlobalTableInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::CreateGlobalTableInputBody {
        global_table_name: &input.global_table_name,
        replication_group: &input.replication_group,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_create_table_input_body(
    input: &crate::input::CreateTableInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::CreateTableInputBody {
        attribute_definitions: &input.attribute_definitions,
        table_name: &input.table_name,
        key_schema: &input.key_schema,
        local_secondary_indexes: &input.local_secondary_indexes,
        global_secondary_indexes: &input.global_secondary_indexes,
        billing_mode: &input.billing_mode,
        provisioned_throughput: &input.provisioned_throughput,
        stream_specification: &input.stream_specification,
        sse_specification: &input.sse_specification,
        tags: &input.tags,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_delete_backup_input_body(
    input: &crate::input::DeleteBackupInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DeleteBackupInputBody {
        backup_arn: &input.backup_arn,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_delete_item_input_body(
    input: &crate::input::DeleteItemInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DeleteItemInputBody {
        table_name: &input.table_name,
        key: &input.key,
        expected: &input.expected,
        conditional_operator: &input.conditional_operator,
        return_values: &input.return_values,
        return_consumed_capacity: &input.return_consumed_capacity,
        return_item_collection_metrics: &input.return_item_collection_metrics,
        condition_expression: &input.condition_expression,
        expression_attribute_names: &input.expression_attribute_names,
        expression_attribute_values: &input.expression_attribute_values,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_delete_table_input_body(
    input: &crate::input::DeleteTableInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DeleteTableInputBody {
        table_name: &input.table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_backup_input_body(
    input: &crate::input::DescribeBackupInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeBackupInputBody {
        backup_arn: &input.backup_arn,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_continuous_backups_input_body(
    input: &crate::input::DescribeContinuousBackupsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeContinuousBackupsInputBody {
        table_name: &input.table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_contributor_insights_input_body(
    input: &crate::input::DescribeContributorInsightsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeContributorInsightsInputBody {
        table_name: &input.table_name,
        index_name: &input.index_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_export_input_body(
    input: &crate::input::DescribeExportInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeExportInputBody {
        export_arn: &input.export_arn,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_global_table_input_body(
    input: &crate::input::DescribeGlobalTableInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeGlobalTableInputBody {
        global_table_name: &input.global_table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_global_table_settings_input_body(
    input: &crate::input::DescribeGlobalTableSettingsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeGlobalTableSettingsInputBody {
        global_table_name: &input.global_table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_kinesis_streaming_destination_input_body(
    input: &crate::input::DescribeKinesisStreamingDestinationInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeKinesisStreamingDestinationInputBody {
        table_name: &input.table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_table_input_body(
    input: &crate::input::DescribeTableInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeTableInputBody {
        table_name: &input.table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_table_replica_auto_scaling_input_body(
    input: &crate::input::DescribeTableReplicaAutoScalingInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeTableReplicaAutoScalingInputBody {
        table_name: &input.table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_describe_time_to_live_input_body(
    input: &crate::input::DescribeTimeToLiveInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DescribeTimeToLiveInputBody {
        table_name: &input.table_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_disable_kinesis_streaming_destination_input_body(
    input: &crate::input::DisableKinesisStreamingDestinationInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::DisableKinesisStreamingDestinationInputBody {
        table_name: &input.table_name,
        stream_arn: &input.stream_arn,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_enable_kinesis_streaming_destination_input_body(
    input: &crate::input::EnableKinesisStreamingDestinationInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::EnableKinesisStreamingDestinationInputBody {
        table_name: &input.table_name,
        stream_arn: &input.stream_arn,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_execute_statement_input_body(
    input: &crate::input::ExecuteStatementInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ExecuteStatementInputBody {
        statement: &input.statement,
        parameters: &input.parameters,
        consistent_read: &input.consistent_read,
        next_token: &input.next_token,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_execute_transaction_input_body(
    input: &crate::input::ExecuteTransactionInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ExecuteTransactionInputBody {
        transact_statements: &input.transact_statements,
        client_request_token: &input.client_request_token,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_export_table_to_point_in_time_input_body(
    input: &crate::input::ExportTableToPointInTimeInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ExportTableToPointInTimeInputBody {
        table_arn: &input.table_arn,
        export_time: &input.export_time,
        client_token: &input.client_token,
        s3_bucket: &input.s3_bucket,
        s3_bucket_owner: &input.s3_bucket_owner,
        s3_prefix: &input.s3_prefix,
        s3_sse_algorithm: &input.s3_sse_algorithm,
        s3_sse_kms_key_id: &input.s3_sse_kms_key_id,
        export_format: &input.export_format,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_get_item_input_body(
    input: &crate::input::GetItemInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::GetItemInputBody {
        table_name: &input.table_name,
        key: &input.key,
        attributes_to_get: &input.attributes_to_get,
        consistent_read: &input.consistent_read,
        return_consumed_capacity: &input.return_consumed_capacity,
        projection_expression: &input.projection_expression,
        expression_attribute_names: &input.expression_attribute_names,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_list_backups_input_body(
    input: &crate::input::ListBackupsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ListBackupsInputBody {
        table_name: &input.table_name,
        limit: &input.limit,
        time_range_lower_bound: &input.time_range_lower_bound,
        time_range_upper_bound: &input.time_range_upper_bound,
        exclusive_start_backup_arn: &input.exclusive_start_backup_arn,
        backup_type: &input.backup_type,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_list_contributor_insights_input_body(
    input: &crate::input::ListContributorInsightsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ListContributorInsightsInputBody {
        table_name: &input.table_name,
        next_token: &input.next_token,
        max_results: &input.max_results,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_list_exports_input_body(
    input: &crate::input::ListExportsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ListExportsInputBody {
        table_arn: &input.table_arn,
        max_results: &input.max_results,
        next_token: &input.next_token,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_list_global_tables_input_body(
    input: &crate::input::ListGlobalTablesInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ListGlobalTablesInputBody {
        exclusive_start_global_table_name: &input.exclusive_start_global_table_name,
        limit: &input.limit,
        region_name: &input.region_name,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_list_tables_input_body(
    input: &crate::input::ListTablesInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ListTablesInputBody {
        exclusive_start_table_name: &input.exclusive_start_table_name,
        limit: &input.limit,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_list_tags_of_resource_input_body(
    input: &crate::input::ListTagsOfResourceInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ListTagsOfResourceInputBody {
        resource_arn: &input.resource_arn,
        next_token: &input.next_token,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_put_item_input_body(
    input: &crate::input::PutItemInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::PutItemInputBody {
        table_name: &input.table_name,
        item: &input.item,
        expected: &input.expected,
        return_values: &input.return_values,
        return_consumed_capacity: &input.return_consumed_capacity,
        return_item_collection_metrics: &input.return_item_collection_metrics,
        conditional_operator: &input.conditional_operator,
        condition_expression: &input.condition_expression,
        expression_attribute_names: &input.expression_attribute_names,
        expression_attribute_values: &input.expression_attribute_values,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_query_input_body(
    input: &crate::input::QueryInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::QueryInputBody {
        table_name: &input.table_name,
        index_name: &input.index_name,
        select: &input.select,
        attributes_to_get: &input.attributes_to_get,
        limit: &input.limit,
        consistent_read: &input.consistent_read,
        key_conditions: &input.key_conditions,
        query_filter: &input.query_filter,
        conditional_operator: &input.conditional_operator,
        scan_index_forward: &input.scan_index_forward,
        exclusive_start_key: &input.exclusive_start_key,
        return_consumed_capacity: &input.return_consumed_capacity,
        projection_expression: &input.projection_expression,
        filter_expression: &input.filter_expression,
        key_condition_expression: &input.key_condition_expression,
        expression_attribute_names: &input.expression_attribute_names,
        expression_attribute_values: &input.expression_attribute_values,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_restore_table_from_backup_input_body(
    input: &crate::input::RestoreTableFromBackupInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::RestoreTableFromBackupInputBody {
        target_table_name: &input.target_table_name,
        backup_arn: &input.backup_arn,
        billing_mode_override: &input.billing_mode_override,
        global_secondary_index_override: &input.global_secondary_index_override,
        local_secondary_index_override: &input.local_secondary_index_override,
        provisioned_throughput_override: &input.provisioned_throughput_override,
        sse_specification_override: &input.sse_specification_override,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_restore_table_to_point_in_time_input_body(
    input: &crate::input::RestoreTableToPointInTimeInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::RestoreTableToPointInTimeInputBody {
        source_table_arn: &input.source_table_arn,
        source_table_name: &input.source_table_name,
        target_table_name: &input.target_table_name,
        use_latest_restorable_time: &input.use_latest_restorable_time,
        restore_date_time: &input.restore_date_time,
        billing_mode_override: &input.billing_mode_override,
        global_secondary_index_override: &input.global_secondary_index_override,
        local_secondary_index_override: &input.local_secondary_index_override,
        provisioned_throughput_override: &input.provisioned_throughput_override,
        sse_specification_override: &input.sse_specification_override,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_scan_input_body(
    input: &crate::input::ScanInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::ScanInputBody {
        table_name: &input.table_name,
        index_name: &input.index_name,
        attributes_to_get: &input.attributes_to_get,
        limit: &input.limit,
        select: &input.select,
        scan_filter: &input.scan_filter,
        conditional_operator: &input.conditional_operator,
        exclusive_start_key: &input.exclusive_start_key,
        return_consumed_capacity: &input.return_consumed_capacity,
        total_segments: &input.total_segments,
        segment: &input.segment,
        projection_expression: &input.projection_expression,
        filter_expression: &input.filter_expression,
        expression_attribute_names: &input.expression_attribute_names,
        expression_attribute_values: &input.expression_attribute_values,
        consistent_read: &input.consistent_read,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_tag_resource_input_body(
    input: &crate::input::TagResourceInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::TagResourceInputBody {
        resource_arn: &input.resource_arn,
        tags: &input.tags,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_transact_get_items_input_body(
    input: &crate::input::TransactGetItemsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::TransactGetItemsInputBody {
        transact_items: &input.transact_items,
        return_consumed_capacity: &input.return_consumed_capacity,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_transact_write_items_input_body(
    input: &crate::input::TransactWriteItemsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::TransactWriteItemsInputBody {
        transact_items: &input.transact_items,
        return_consumed_capacity: &input.return_consumed_capacity,
        return_item_collection_metrics: &input.return_item_collection_metrics,
        client_request_token: &input.client_request_token,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_untag_resource_input_body(
    input: &crate::input::UntagResourceInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UntagResourceInputBody {
        resource_arn: &input.resource_arn,
        tag_keys: &input.tag_keys,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_continuous_backups_input_body(
    input: &crate::input::UpdateContinuousBackupsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateContinuousBackupsInputBody {
        table_name: &input.table_name,
        point_in_time_recovery_specification: &input.point_in_time_recovery_specification,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_contributor_insights_input_body(
    input: &crate::input::UpdateContributorInsightsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateContributorInsightsInputBody {
        table_name: &input.table_name,
        index_name: &input.index_name,
        contributor_insights_action: &input.contributor_insights_action,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_global_table_input_body(
    input: &crate::input::UpdateGlobalTableInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateGlobalTableInputBody {
        global_table_name: &input.global_table_name,
        replica_updates: &input.replica_updates,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_global_table_settings_input_body(
    input: &crate::input::UpdateGlobalTableSettingsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateGlobalTableSettingsInputBody {
        global_table_name: &input.global_table_name,
        global_table_billing_mode: &input.global_table_billing_mode,
        global_table_provisioned_write_capacity_units: &input
            .global_table_provisioned_write_capacity_units,
        global_table_provisioned_write_capacity_auto_scaling_settings_update: &input
            .global_table_provisioned_write_capacity_auto_scaling_settings_update,
        global_table_global_secondary_index_settings_update: &input
            .global_table_global_secondary_index_settings_update,
        replica_settings_update: &input.replica_settings_update,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_item_input_body(
    input: &crate::input::UpdateItemInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateItemInputBody {
        table_name: &input.table_name,
        key: &input.key,
        attribute_updates: &input.attribute_updates,
        expected: &input.expected,
        conditional_operator: &input.conditional_operator,
        return_values: &input.return_values,
        return_consumed_capacity: &input.return_consumed_capacity,
        return_item_collection_metrics: &input.return_item_collection_metrics,
        update_expression: &input.update_expression,
        condition_expression: &input.condition_expression,
        expression_attribute_names: &input.expression_attribute_names,
        expression_attribute_values: &input.expression_attribute_values,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_table_input_body(
    input: &crate::input::UpdateTableInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateTableInputBody {
        attribute_definitions: &input.attribute_definitions,
        table_name: &input.table_name,
        billing_mode: &input.billing_mode,
        provisioned_throughput: &input.provisioned_throughput,
        global_secondary_index_updates: &input.global_secondary_index_updates,
        stream_specification: &input.stream_specification,
        sse_specification: &input.sse_specification,
        replica_updates: &input.replica_updates,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_table_replica_auto_scaling_input_body(
    input: &crate::input::UpdateTableReplicaAutoScalingInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateTableReplicaAutoScalingInputBody {
        global_secondary_index_updates: &input.global_secondary_index_updates,
        table_name: &input.table_name,
        provisioned_write_capacity_auto_scaling_update: &input
            .provisioned_write_capacity_auto_scaling_update,
        replica_updates: &input.replica_updates,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

pub fn serialize_operation_update_time_to_live_input_body(
    input: &crate::input::UpdateTimeToLiveInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let body = crate::serializer::UpdateTimeToLiveInputBody {
        table_name: &input.table_name,
        time_to_live_specification: &input.time_to_live_specification,
    };
    serde_json::to_vec(&body).map(smithy_http::body::SdkBody::from)
}

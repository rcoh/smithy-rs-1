/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use std::error::Error;

use dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use dynamodb::operation::CreateTable;
use dynamodb::output::{ListTablesOutput, DeleteTableOutput};
use operation::{HttpRequestResponse, SdkBody, Operation};
use dynamodb::error::DeleteTableError;

struct DeleteTable(dynamodb::operation::DeleteTable);

use bytes::Bytes;
use auth::{SigningConfig, HttpSigningConfig, SigningAlgorithm, HttpSignatureType, ServiceConfig, RequestConfig};
use std::time::SystemTime;
use operation::endpoint::StaticEndpoint;
use http::{Response, Uri};

impl HttpRequestResponse for DeleteTable {
    type O = Result<DeleteTableOutput, DeleteTableError>;

    fn parse_unloaded<B>(&self, _: &mut Response<B>) -> Option<Self::O> {
        None
    }

    fn parse_loaded(&self, response: &Response<Bytes>) -> Self::O {
        self.0.parse_response(response)
    }
}

impl DeleteTable {
    fn into_operation(self, config: dynamodb::Config) -> Operation<DeleteTable> {
        Operation {
            base: self.0.build_http_request().map(|body|SdkBody::from(body)),
            signing_config: SigningConfig::Http(HttpSigningConfig {
                algorithm: SigningAlgorithm::SigV4,
                signature_type: HttpSignatureType::HttpRequestHeaders,
                service_config: ServiceConfig {
                    // TODO: these get loaded from the config
                    service: "dynamodb".to_string(),
                    region: "us-east-1".to_string(),
                },
                request_config: RequestConfig {
                    request_ts: || SystemTime::now(),
                },
                double_uri_encode: false,
                normalize_uri_path: true,
                omit_session_token: false,
            }),
            credentials_provider: config.credentials_provider,
            endpoint_config: Box::new(StaticEndpoint::from_uri(Uri::from_static("http://localhost:8000"))),
            response_handler: Some(Box::new(self)),
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let table_name = "new_table";
    let client = aws_hyper::Client::default();
    let config = dynamodb::Config::builder().build();
    let clear_table = dynamodb::operation::DeleteTable::builder()
        .table_name(table_name)
        .build(&config);
    let clear_table = DeleteTable(clear_table);
    match client.call(clear_table.into_operation(config)).await {
        Ok(response) => println!("OK! {:?}", response.parsed),
        Err(_response) => println!("failure: {}", _response.error())
    }

    /*
    match io_v0::dispatch!(client, clear_table).parsed() {
        Ok(Ok(table_deleted)) => println!(
            "{:?} was deleted",
            table_deleted
                .table_description
                .as_ref()
                .unwrap()
                .table_name
                .as_ref()
                .unwrap()
        ),
        Ok(Err(table_del_error)) => println!("failed to delete table: {}", table_del_error),
        Err(e) => println!("dispatch error: {:?}", e),
    }

    let tables = io_v0::dispatch!(
        client,
        dynamodb::operation::ListTables::builder().build(&config)
    )
    .parsed
    .unwrap();
    assert_eq!(
        tables.unwrap(),
        ListTablesOutput::builder().table_names(vec![]).build()
    );
    println!("no tables...creating table");

    let create_table = CreateTable::builder()
        .table_name(table_name)
        .attribute_definitions(vec![AttributeDefinition::builder()
            .attribute_name("ForumName")
            .attribute_type(ScalarAttributeType::S)
            .build()])
        .key_schema(vec![KeySchemaElement::builder()
            .attribute_name("ForumName")
            .key_type(KeyType::Hash)
            .build()])
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(100)
                .write_capacity_units(100)
                .build(),
        )
        .build(&config);

    let response = io_v0::dispatch!(client, create_table);
    match response.parsed {
        Some(Ok(output)) => {
            assert_eq!(
                output.table_description.unwrap().table_name.unwrap(),
                table_name
            );
            println!("{} was created", table_name);
        }
        _ => println!("{:?}", response.raw),
    }

    let tables = io_v0::dispatch!(
        client,
        dynamodb::operation::ListTables::builder().build(&config)
    )
    .parsed
    .unwrap();
    println!(
        "current tables: {:?}",
        &tables.as_ref().unwrap().table_names
    );
    assert_eq!(
        tables.unwrap(),
        ListTablesOutput::builder()
            .table_names(vec![table_name.to_string()])
            .build()
    );*/

    Ok(())
}
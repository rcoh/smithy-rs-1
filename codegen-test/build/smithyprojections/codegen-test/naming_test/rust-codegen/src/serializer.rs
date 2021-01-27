// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use crate::model::String;
use crate::model::Vec;
#[non_exhaustive]
#[derive(::serde::Serialize, ::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReservedWordsAsMembersInputBody<'a> {
    #[serde(rename = "as")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#as: &'a ::std::option::Option<i32>,
    #[serde(rename = "async")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#async: &'a ::std::option::Option<bool>,
}

#[non_exhaustive]
#[derive(::serde::Serialize, ::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StructureNamePunningInputBody<'a> {
    #[serde(rename = "regular_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_string: &'a ::std::option::Option<::std::string::String>,
    #[serde(rename = "punned_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punned_string: &'a ::std::option::Option<String>,
    #[serde(rename = "punned_vec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punned_vec: &'a ::std::option::Option<Vec>,
}
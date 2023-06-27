#![allow(unused)]
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GetOperationResult {
    operation_id: Option<String>,
    status: Option<Status>,
    percent_completed: Option<u32>,
    created_date_time: Option<String>,
    last_updated_date_time: Option<String>,
    kind: Option<OperationKind>,
    resource_location: Option<String>,
    api_version: Option<String>,

    // Tags keys cannot be known at compile time.
    tags: serde_json::Value,

    // Result field is not documented.
    result: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ListOperationsResponse {
    value: Option<Vec<OperationInfo>>,
    next_link: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OperationInfo {
    operation_id: Option<String>,
    status: Option<Status>,
    percent_completed: Option<u32>,
    created_date_time: Option<String>,
    last_updated_date_time: Option<String>,
    kind: Option<OperationKind>,
    resource_location: Option<String>,
    api_version: Option<String>,
    // Tag keys cannot be known at compile time.
    tags: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum Status {
    NotStarted,
    Running,
    Failed,
    Succeeded,
    Canceled,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum OperationKind {
    DocumentModelBuild,
    DocumentModelCompose,
    DocumentModelCopyTo,
}
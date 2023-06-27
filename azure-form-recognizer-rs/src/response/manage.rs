#![allow(unused)]
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GenCopyAuthResponse {
    target_resource_id: Option<String>,
    target_resource_region: Option<String>,
    target_model_id: Option<String>,
    target_model_location: Option<String>,
    access_token: Option<String>,
    expiration_date_time: Option<String>
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GetInfoResponse {
    custom_document_models: Option<CustomModelInfo>
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CustomModelInfo {
    count: Option<u32>,
    limit: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GetModelResponse {
    model_id: Option<String>,
    description: Option<String>,
    created_date_time: Option<String>,
    api_version: Option<String>,
    // doc_type keys and tag keys cannot be known at compile time.
    doc_types: serde_json::Value,
    tags: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ListModelsResponse {
    value: Option<Vec<ModelInfo>>,
    next_link: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ModelInfo {
    model_id: Option<String>,
    created_date_time: Option<String>,
    api_version: Option<String>,
    description: Option<String>,
    // Tag keys cannot be known at compile time.
    tags: serde_json::Value,
}
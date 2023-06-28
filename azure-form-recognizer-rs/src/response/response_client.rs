//https://westus.dev.cognitive.microsoft.com/docs/services/form-recognizer-api-2022-08-31/operations/AnalyzeDocument

use reqwest::{Client, ClientBuilder, Method, header};
use std::error::Error;
use std::collections::HashMap;
use std::io::Read;
use url::Url;
use core::future::Future;

use crate::request::request_client::PostAnalyzeResult;

pub struct ResponseClient {
    client: reqwest::Client,
    endpoint: String,
}

impl ResponseClient {
    pub fn new<T: ToString>(key: &[u8], endpoint: T) -> Result<Self, Box<dyn Error>> {
        let mut auth_header = header::HeaderMap::new();
        auth_header.insert(
            header::HeaderName::from_static("ocp-apim-subscription-key"), 
            header::HeaderValue::from_bytes(key)?
        );

        let client = Self {
            client: Client::builder()
                .default_headers(auth_header)
                .build()?,
            endpoint: endpoint.to_string(),
        };
        
        Ok(client)
    }

    // TODO: Make this an impl of GetAnalyzeResult. Use Reqwest errors as the source for StatusCodeError, or pass reqwest::Error directly??? Except the request was successful.
    fn check_request_status_code(analyze_doc_response: &PostAnalyzeResult) -> Result<(), StatusCodeError> {
        let status = analyze_doc_response.0.status().as_u16();
        match status {
            202 => Ok(()),
            400..=499 => Err(StatusCodeError::FatalError),
            500..=599 => Err(StatusCodeError::TransientError),
            _ => Err(StatusCodeError::FatalError)
        }
    }

    // Panics if Operation-Location header is not present (which can happen if a bad request is passed in)
    fn parse_operation_location(analyze_doc_response: &PostAnalyzeResult) -> Result<String, reqwest::header::ToStrError> {
        let operation_location = analyze_doc_response.0
            .headers()
            .get("Operation-Location")
            .expect("Could not find Operation-Location key")
            .to_str()?
            .to_owned();
        Ok(operation_location)
    }

    pub async fn get_analyze_result(&self, analyze_doc_response: &PostAnalyzeResult)
        -> Result<GetAnalyzeResult, Box<dyn Error>> {
        Self::check_request_status_code(analyze_doc_response)?;
        let request_url = Self::parse_operation_location(analyze_doc_response)?;
        let response = self.client
            .get(request_url)
            .send()
            .await;
        Ok(GetAnalyzeResult(response?))
    }
}

pub struct GetAnalyzeResult(pub reqwest::Response);

use super::analyze::GetAnalyzeResultResponse;
impl GetAnalyzeResult {
    async fn read_body(&mut self) -> Result<GetAnalyzeResultResponse, reqwest::Error> {
        self.0.json::<GetAnalyzeResultResponse>().await
    }
}

#[derive(Debug)]
pub enum StatusCodeError {
    TransientError,
    FatalError,
}

// TODO: Implement this error better. Error struct syntax? self.source?
impl std::fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::FatalError => write!(f, "detected a fatal error in AnalyzeDocument response (typically 4XX or unexpected status code)"),
            Self::TransientError => write!(f, "detected a potentially recoverable error in AnalyzeDocument response (5XX status code)")
        }
    }
}

impl Error for StatusCodeError {
}
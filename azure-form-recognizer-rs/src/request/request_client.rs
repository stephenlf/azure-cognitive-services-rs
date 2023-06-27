use reqwest::{Client, ClientBuilder, Method, header};
use std::error::Error;
use std::collections::HashMap;
use std::io::Read;
use url::Url;
use core::future::Future;

pub struct RequestClient {
    client: reqwest::Client,
    endpoint: String,
}

impl RequestClient {
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

    fn build_analyze_uri(&self, model_id: &str, optional_params: OptionalParams) -> String {
        let mut request_url = self.endpoint.clone();
        request_url.push_str("/formrecognizer/documentModels/");
        request_url.push_str(model_id);
        request_url.push_str(":analyze?api-version=2022-08-31");
        if let Some(s) = optional_params.pages {
            request_url.push_str("&pages=");
            request_url.push_str(s.as_str());
        }
        if let Some(s) = optional_params.locale {
            request_url.push_str("&locale=");
            request_url.push_str(s.as_str());
        }
        if let Some(s) = optional_params.string_index_type {
            request_url.push_str("&stringIndexType=");
            request_url.push_str(s.as_str());
        }
        request_url
    }

    pub async fn analyze_url<T: ToString>(&self, document_url: T, model_id: &str, optional_params: OptionalParams)
        -> Result<PostAnalyzeResult, reqwest::Error> {
        let request_url = Self::build_analyze_uri(&self, model_id, optional_params);

        let body = format!("{{ \"urlSource\": {:?} }}", document_url.to_string());

        let request = self.client.request(Method::POST, request_url)
            .header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"))
            .body(body)
            .send();

        Ok(PostAnalyzeResult(request.await?))
    }

    /// Returns a future 
    pub async fn analyze_file(&self, file_bytes: Vec<u8>, file_type: FileType, model_id: &str, optional_params: OptionalParams) 
        -> impl Future <Output = Result<reqwest::Response, reqwest::Error>> {
        
        let request_url = Self::build_analyze_uri(&self, model_id, optional_params);

        let header_value = match file_type {
            FileType::OctetStream => "application/octet-stream",
            FileType::PDF => "application/pdf",
            FileType::JPEG => "image/jpeg",
            FileType::PNG => "image/png",
            FileType::TIFF => "image/tiff",
            FileType::BMP => "image/bmp",
            FileType::HTML => "text/html",
            FileType::DOCX => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            FileType::XLSX => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            FileType::PPT => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        };
        
        let request = self.client.request(Method::POST, request_url)
            .header(header::CONTENT_TYPE, header::HeaderValue::from_static(header_value))
            .body(file_bytes)
            .send();

        request
    }
}

pub struct PostAnalyzeResult(pub reqwest::Response);

pub struct OptionalParams {
    pages: Option<String>,
    locale: Option<String>,
    string_index_type: Option<String>,
}

impl Default for OptionalParams {
    fn default() -> Self {
        OptionalParams { pages: None, locale: None, string_index_type: None }
    }
}

pub enum FileType {
    OctetStream,
    PDF,
    JPEG,
    PNG,
    TIFF,
    BMP,
    HTML,
    DOCX,
    XLSX,
    PPT,
}
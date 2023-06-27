use reqwest::{Client, ClientBuilder, Method, header};
use std::error::Error;
use std::collections::HashMap;
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

        let client = RequestClient {
            client: Client::builder()
                .default_headers(auth_header)
                .build()?,
            endpoint: endpoint.to_string(),
        };
        
        Ok(client)
    }

    pub fn analyze_url<T: ToString>(&self, document_url: T, model_id: &str, optional_params: OptionalParams)
        -> Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>>> {
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

        let body = format!("{{ \"urlSource\": {:?} }}", document_url.to_string());

        let request = self.client.request(Method::POST, request_url)
            .header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"))
            .body(body)
            .send();

        Box::new(request)
            
    }
}

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
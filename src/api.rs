use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::{anyhow, Result};

#[derive(Serialize)]
pub struct ApiRequest<'a> {
    pub query: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeloResponseData {
    pub id: String,
    pub message_id: String,
    pub answer: String, // Renamed from markdown
    pub query_analysis: Option<serde_json::Value>,
    pub resources: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeloApiResponse {
    pub status: serde_json::Value,
    pub message: Option<String>,
    pub data: Option<FeloResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeloErrorResponse {
    pub status: String,
    pub code: String,
    pub message: String,
    pub request_id: Option<String>,
}

pub async fn call_felo_api(api_key: &str, query: &str) -> Result<FeloResponseData> {
    let client = Client::new();
    let request_body = ApiRequest { query };

    let response = client
        .post("https://openapi.felo.ai/v2/chat")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    let status_code = response.status();
    let full_response_text = response.text().await?;

    if !status_code.is_success() {
        // Try to parse into FeloErrorResponse if it's an error from the API
        if let Ok(felo_error) = serde_json::from_str::<FeloErrorResponse>(&full_response_text) {
            return Err(anyhow!(
                "API Error ({} {}): Code: {}, Message: {}, Request ID: {:?}",
                status_code,
                felo_error.status,
                felo_error.code,
                felo_error.message,
                felo_error.request_id
            ));
        } else {
            // Fallback for unexpected error formats
            return Err(anyhow!(
                "API request failed with status: {}. Body: {}",
                status_code,
                full_response_text
            ));
        }
    }

    // Try to parse into FeloApiResponse
    let felo_api_response: FeloApiResponse = serde_json::from_str(&full_response_text)?;

    let status_ok = if let Some(s) = felo_api_response.status.as_str() {
        s == "ok"
    } else if let Some(n) = felo_api_response.status.as_u64() {
        n == 200
    } else {
        false
    };

    if status_ok {
        if let Some(data) = felo_api_response.data {
            Ok(data)
        } else {
            Err(anyhow!("API response 'data' field is missing for successful request."))
        }
    } else {
        // This case should ideally be caught by !status_code.is_success()
        // but acts as a safeguard if the API returns non-200 with status "error"
        Err(anyhow!(
            "API returned status '{}': {}",
            felo_api_response.status,
            felo_api_response.message.unwrap_or_else(|| "No message provided".to_string())
        ))
    }
}


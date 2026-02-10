use async_trait::async_trait;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::application::ports::{ChatGateway, GatewayError, HealthGateway};
use crate::domain::{ApiBaseUrl, AskResult, HealthStatus, Question};

#[derive(Clone, Debug)]
pub struct ApiClient {
    base_url: ApiBaseUrl,
}

impl ApiClient {
    pub fn new(base_url: ApiBaseUrl) -> Self {
        Self { base_url }
    }

    fn endpoint(&self, path: &str) -> String {
        self.base_url.join(path)
    }
}

#[derive(Debug, Serialize)]
struct AskRequestDto {
    question: String,
}

#[derive(Debug, Deserialize)]
struct AskResponseDto {
    answer: String,
    source: String,
    system_prompt_applied: bool,
}

#[derive(Debug, Deserialize)]
struct HealthResponseDto {
    status: String,
    version: String,
    gigachat_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct ErrorResponseDto {
    error: String,
    code: Option<String>,
}

#[async_trait(?Send)]
impl ChatGateway for ApiClient {
    async fn ask(&self, question: Question) -> Result<AskResult, GatewayError> {
        let url = self.endpoint("/ask");
        let payload = AskRequestDto {
            question: question.as_str().to_string(),
        };

        let response = Request::post(&url)
            .json(&payload)
            .map_err(|e| GatewayError::Network(e.to_string()))?
            .send()
            .await
            .map_err(|e| GatewayError::Network(e.to_string()))?;

        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        if !response.ok() {
            if let Ok(error_payload) = serde_json::from_str::<ErrorResponseDto>(&text) {
                let code = error_payload.code.unwrap_or_else(|| "unknown".to_string());
                return Err(GatewayError::Api(format!(
                    "{} (код: {})",
                    error_payload.error, code
                )));
            }
            return Err(GatewayError::Api(format!("HTTP {}: {}", status, text)));
        }

        let payload = match serde_json::from_str::<AskResponseDto>(&text) {
            Ok(parsed) => parsed,
            Err(_) => {
                if let Ok(error_payload) = serde_json::from_str::<ErrorResponseDto>(&text) {
                    let code = error_payload.code.unwrap_or_else(|| "unknown".to_string());
                    return Err(GatewayError::Api(format!(
                        "{} (код: {})",
                        error_payload.error, code
                    )));
                }
                return Err(GatewayError::InvalidPayload);
            }
        };

        Ok(AskResult {
            answer: payload.answer,
            source: payload.source,
            system_prompt_applied: payload.system_prompt_applied,
        })
    }
}

#[async_trait(?Send)]
impl HealthGateway for ApiClient {
    async fn health(&self) -> Result<HealthStatus, GatewayError> {
        let url = self.endpoint("/health");
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| GatewayError::Network(e.to_string()))?;

        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        if !response.ok() {
            return Err(GatewayError::Api(format!("HTTP {}: {}", status, text)));
        }

        let payload: HealthResponseDto =
            serde_json::from_str(&text).map_err(|_| GatewayError::InvalidPayload)?;

        Ok(HealthStatus {
            status: payload.status,
            version: payload.version,
            gigachat_enabled: payload.gigachat_enabled,
        })
    }
}

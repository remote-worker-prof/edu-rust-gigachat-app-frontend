use gloo_storage::{LocalStorage, Storage};

use crate::domain::{ApiBaseUrl, DomainError};

const STORAGE_KEY: &str = "rust_gigachat_webapp.api_base_url";

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub api_base_url: ApiBaseUrl,
}

impl AppConfig {
    pub fn load() -> Self {
        if let Ok(saved) = LocalStorage::get::<String>(STORAGE_KEY) {
            if let Ok(base_url) = ApiBaseUrl::try_new(saved) {
                return Self { api_base_url: base_url };
            }
        }

        let fallback = Self::default_base_url();
        let base_url = ApiBaseUrl::try_new(fallback)
            .unwrap_or_else(|_| ApiBaseUrl::try_new("http://127.0.0.1:8000").expect("default URL"));

        Self { api_base_url: base_url }
    }

    pub fn default_base_url() -> String {
        option_env!("API_BASE_URL")
            .unwrap_or("http://127.0.0.1:8000")
            .to_string()
    }

    pub fn save_base_url(base_url: &ApiBaseUrl) -> Result<(), String> {
        LocalStorage::set(STORAGE_KEY, base_url.as_str().to_string())
            .map_err(|e| e.to_string())
    }

    pub fn parse_base_url(value: &str) -> Result<ApiBaseUrl, DomainError> {
        ApiBaseUrl::try_new(value.to_string())
    }
}

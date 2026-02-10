//! Конфигурация UI‑клиента.
//!
//! Основная идея: базовый URL API можно задавать гибко.
//! Источники значения идут по приоритету:
//! 1) сохранённое в `localStorage`,
//! 2) переменная окружения `API_BASE_URL` на этапе сборки,
//! 3) значение по умолчанию (`http://127.0.0.1:8000`).
use gloo_storage::{LocalStorage, Storage};

use crate::domain::{ApiBaseUrl, DomainError};

const STORAGE_KEY: &str = "rust_gigachat_webapp.api_base_url";

/// Конфигурация приложения, доступная UI‑слою.
///
/// Сейчас она хранит только базовый URL API, но структура оставлена
/// расширяемой — в будущем можно добавить таймауты, флаги режима и т.п.
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub api_base_url: ApiBaseUrl,
}

impl AppConfig {
    /// Загружает конфигурацию, следуя приоритетам источников.
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

    /// Возвращает базовый URL по умолчанию.
    ///
    /// Если во время сборки задана переменная окружения `API_BASE_URL`,
    /// она имеет приоритет.
    pub fn default_base_url() -> String {
        option_env!("API_BASE_URL")
            .unwrap_or("http://127.0.0.1:8000")
            .to_string()
    }

    /// Сохраняет базовый URL в `localStorage`.
    ///
    /// Возвращает строку ошибки, чтобы UI мог показать её пользователю.
    pub fn save_base_url(base_url: &ApiBaseUrl) -> Result<(), String> {
        LocalStorage::set(STORAGE_KEY, base_url.as_str().to_string())
            .map_err(|e| e.to_string())
    }

    /// Валидирует ввод пользователя и преобразует строку в value object.
    pub fn parse_base_url(value: &str) -> Result<ApiBaseUrl, DomainError> {
        ApiBaseUrl::try_new(value.to_string())
    }
}

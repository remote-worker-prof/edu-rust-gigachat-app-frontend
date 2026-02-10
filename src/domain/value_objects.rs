//! Value objects предметной области.
//!
//! Value object — это тип, который инкапсулирует правила валидности данных.
//! В нашем проекте это:
//! - `Question` — вопрос пользователя;
//! - `ApiBaseUrl` — базовый URL API.
use super::errors::DomainError;

/// Вопрос пользователя.
///
/// Правило: вопрос не должен быть пустым.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question(String);

impl Question {
    /// Создаёт вопрос и валидирует его содержимое.
    pub fn try_new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::EmptyQuestion);
        }
        Ok(Self(value))
    }

    /// Возвращает исходный текст вопроса.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Базовый URL API.
///
/// Правило: строка не должна быть пустой. Дополнительную проверку
/// (например, корректность схемы) можно добавить позже.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiBaseUrl(String);

impl ApiBaseUrl {
    /// Создаёт базовый URL и валидирует его.
    pub fn try_new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::EmptyBaseUrl);
        }
        Ok(Self(value))
    }

    /// Возвращает строковое представление URL.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Склеивает базовый URL и путь эндпоинта.
    ///
    /// Метод нормализует слеши, чтобы `base/` + `/health` корректно
    /// превращались в `base/health`.
    pub fn join(&self, path: &str) -> String {
        let base = self.0.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{}/{}", base, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_requires_text() {
        assert!(Question::try_new("   ").is_err());
        assert!(Question::try_new("Что такое Rust?").is_ok());
    }

    #[test]
    fn api_base_url_join_handles_slashes() {
        let base = ApiBaseUrl::try_new("http://localhost:8000/").unwrap();
        assert_eq!(base.join("/health"), "http://localhost:8000/health");
        assert_eq!(base.join("ask"), "http://localhost:8000/ask");
    }
}

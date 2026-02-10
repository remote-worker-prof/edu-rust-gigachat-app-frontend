//! Ошибки предметной области.
//!
//! Эти ошибки возникают при нарушении бизнес‑правил.
//! Их важно отделять от сетевых или системных ошибок.
use thiserror::Error;

/// Ошибки домена.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("Вопрос не должен быть пустым")]
    EmptyQuestion,
    #[error("Базовый URL API не задан")]
    EmptyBaseUrl,
}

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("Вопрос не должен быть пустым")]
    EmptyQuestion,
    #[error("Базовый URL API не задан")]
    EmptyBaseUrl,
}

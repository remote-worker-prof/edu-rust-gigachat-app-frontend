use async_trait::async_trait;
use thiserror::Error;

use crate::domain::{AskResult, HealthStatus, Question};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GatewayError {
    #[error("Сетевая ошибка: {0}")]
    Network(String),
    #[error("Ошибка API: {0}")]
    Api(String),
    #[error("Некорректный ответ API")]
    InvalidPayload,
}

#[async_trait(?Send)]
pub trait ChatGateway {
    async fn ask(&self, question: Question) -> Result<AskResult, GatewayError>;
}

#[async_trait(?Send)]
pub trait HealthGateway {
    async fn health(&self) -> Result<HealthStatus, GatewayError>;
}

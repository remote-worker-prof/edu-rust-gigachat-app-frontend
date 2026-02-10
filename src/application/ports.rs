//! Порты (интерфейсы) слоя application.
//!
//! Порт — это контракт, описывающий, что нужно приложению, не привязываясь
//! к конкретной реализации. В учебном проекте такими портами являются:
//! - отправка вопроса (`ChatGateway`);
//! - проверка статуса (`HealthGateway`).
use async_trait::async_trait;
use thiserror::Error;

use crate::domain::{AskResult, HealthStatus, Question};

/// Ошибки уровня шлюза (gateway).
///
/// Эти ошибки отражают взаимодействие с внешней системой (API).
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GatewayError {
    #[error("Сетевая ошибка: {0}")]
    Network(String),
    #[error("Ошибка API: {0}")]
    Api(String),
    #[error("Некорректный ответ API")]
    InvalidPayload,
}

/// Порт для отправки вопроса в backend.
#[async_trait(?Send)]
pub trait ChatGateway {
    async fn ask(&self, question: Question) -> Result<AskResult, GatewayError>;
}

/// Порт для проверки состояния backend.
#[async_trait(?Send)]
pub trait HealthGateway {
    async fn health(&self) -> Result<HealthStatus, GatewayError>;
}

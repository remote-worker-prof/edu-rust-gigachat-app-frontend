//! Use‑cases (сценарии использования) приложения.
//!
//! Каждая структура в этом файле описывает конкретный сценарий:
//! - `AskQuestionUseCase` — отправка вопроса;
//! - `CheckHealthUseCase` — проверка доступности API.
//!
//! Use‑cases используют только порты, поэтому их легко тестировать с фейковыми
//! реализациями.
use thiserror::Error;

use crate::application::ports::{ChatGateway, GatewayError, HealthGateway};
use crate::domain::{AskResult, DomainError, HealthStatus, Question};

/// Ошибка сценария использования.
///
/// Делит ошибки на два типа:
/// - **Domain** — нарушения правил предметной области;
/// - **Gateway** — проблемы взаимодействия с API.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum UseCaseError {
    #[error("Ошибка домена: {0}")]
    Domain(DomainError),
    #[error("Ошибка шлюза: {0}")]
    Gateway(GatewayError),
}

/// Сценарий "задать вопрос".
///
/// Принимает строку, проверяет её на валидность и передаёт в gateway.
pub struct AskQuestionUseCase<G: ChatGateway> {
    gateway: G,
}

impl<G: ChatGateway> AskQuestionUseCase<G> {
    /// Создаёт use‑case с заданной реализацией gateway.
    pub fn new(gateway: G) -> Self {
        Self { gateway }
    }

    /// Выполняет сценарий: валидирует вопрос и отправляет его в API.
    pub async fn execute(&self, question: String) -> Result<AskResult, UseCaseError> {
        let question = Question::try_new(question).map_err(UseCaseError::Domain)?;
        self.gateway
            .ask(question)
            .await
            .map_err(UseCaseError::Gateway)
    }
}

/// Сценарий "проверить состояние API".
pub struct CheckHealthUseCase<G: HealthGateway> {
    gateway: G,
}

impl<G: HealthGateway> CheckHealthUseCase<G> {
    /// Создаёт use‑case с заданной реализацией gateway.
    pub fn new(gateway: G) -> Self {
        Self { gateway }
    }

    /// Выполняет сценарий проверки статуса.
    pub async fn execute(&self) -> Result<HealthStatus, UseCaseError> {
        self.gateway
            .health()
            .await
            .map_err(UseCaseError::Gateway)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use futures::executor::block_on;

    #[derive(Clone)]
    struct FakeChatGateway {
        result: Result<AskResult, GatewayError>,
    }

    #[async_trait(?Send)]
    impl ChatGateway for FakeChatGateway {
        async fn ask(&self, _question: Question) -> Result<AskResult, GatewayError> {
            self.result.clone()
        }
    }

    #[derive(Clone)]
    struct FakeHealthGateway {
        result: Result<HealthStatus, GatewayError>,
    }

    #[async_trait(?Send)]
    impl HealthGateway for FakeHealthGateway {
        async fn health(&self) -> Result<HealthStatus, GatewayError> {
            self.result.clone()
        }
    }

    #[test]
    fn ask_usecase_rejects_empty_question() {
        let gateway = FakeChatGateway {
            result: Ok(AskResult {
                answer: "ok".to_string(),
                source: "mock".to_string(),
                system_prompt_applied: false,
            }),
        };
        let usecase = AskQuestionUseCase::new(gateway);
        let error = block_on(usecase.execute(" ".to_string())).unwrap_err();
        assert!(matches!(error, UseCaseError::Domain(DomainError::EmptyQuestion)));
    }

    #[test]
    fn health_usecase_returns_result() {
        let gateway = FakeHealthGateway {
            result: Ok(HealthStatus {
                status: "ok".to_string(),
                version: "0.1.0".to_string(),
                gigachat_enabled: false,
            }),
        };
        let usecase = CheckHealthUseCase::new(gateway);
        let result = block_on(usecase.execute()).unwrap();
        assert_eq!(result.status, "ok");
    }
}

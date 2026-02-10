//! Слой application (сценарии использования).
//!
//! Здесь нет UI и нет сетевых деталей. В этом слое описываются:
//! - **порты** (интерфейсы), через которые приложение общается с внешним миром;
//! - **use‑cases** — конкретные сценарии вроде «задать вопрос» или «проверить статус».
pub mod ports;
pub mod usecases;

pub use usecases::{AskQuestionUseCase, CheckHealthUseCase, UseCaseError};

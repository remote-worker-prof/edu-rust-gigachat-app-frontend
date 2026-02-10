//! Доменные модели учебного проекта.
//!
//! Домен содержит минимальный набор сущностей и правил:
//! - вопросы не должны быть пустыми;
//! - базовый URL должен быть задан.
//!
//! Эти правила независимы от UI и сети, поэтому домен легко тестировать.
pub mod entities;
pub mod errors;
pub mod value_objects;

pub use entities::{AskResult, HealthStatus};
pub use errors::DomainError;
pub use value_objects::{ApiBaseUrl, Question};

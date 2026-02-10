/// Состояние backend‑сервера.
///
/// Используется для экрана "Статус API".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub gigachat_enabled: bool,
}

/// Результат ответа на вопрос.
///
/// Содержит текст ответа, источник (mock или gigachat) и флаг применения
/// системного промпта.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AskResult {
    pub answer: String,
    pub source: String,
    pub system_prompt_applied: bool,
}

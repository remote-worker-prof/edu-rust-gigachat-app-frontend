#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub gigachat_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AskResult {
    pub answer: String,
    pub source: String,
    pub system_prompt_applied: bool,
}

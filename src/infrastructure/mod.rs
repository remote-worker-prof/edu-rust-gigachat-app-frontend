//! Слой инфраструктуры.
//!
//! Здесь находятся адаптеры и технические детали, которые не должны
//! проникать в домен или use‑cases. В текущем проекте это HTTP‑клиент,
//! реализующий порты `ChatGateway` и `HealthGateway`.
pub mod api_client;

pub use api_client::ApiClient;

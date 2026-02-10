//! Учебный UI‑клиент к API `edu-rust-gigachat-app-backend`.
//!
//! Этот crate собирается в WebAssembly и запускается в браузере. Он демонстрирует
//! разделение по слоям в духе DDD:
//!
//! - **domain** — предметная область (сущности, value objects, ошибки).
//! - **application** — сценарии использования и порты (интерфейсы) для внешних
//!   систем.
//! - **infrastructure** — реализация портов (HTTP‑клиент к backend).
//! - **app** — UI‑слой на Yew, который связывает всё вместе.
//!
//! Основная точка входа для UI — компонент `App`, который экспортируется из
//! этого модуля и используется в `main.rs`.
mod app;
mod application;
mod config;
mod domain;
mod infrastructure;

pub use app::App;

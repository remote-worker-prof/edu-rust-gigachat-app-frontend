//! Точка входа браузерного приложения.
//!
//! Файл минимален: он находит элемент `#app` в HTML и монтирует туда Yew‑компонент
//! `App`. Такая структура облегчает обучение: UI‑логика остаётся в `src/app.rs`,
//! а `main.rs` отвечает только за старт.
use rust_gigachat_webapp::App;

fn main() {
    let window = web_sys::window().expect("window is unavailable");
    let document = window.document().expect("document is unavailable");
    let root = document
        .get_element_by_id("app")
        .expect("missing #app element");

    yew::Renderer::<App>::with_root(root).render();
}

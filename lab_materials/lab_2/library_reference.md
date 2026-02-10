# Справочные материалы по зависимостям (UI‑проект)

Документ содержит краткое описание основных библиотек и инструментов,
используемых в учебном UI‑проекте `rust-gigachat-webapp`.

## 1. Yew

**Назначение:** UI‑фреймворк для создания веб‑интерфейсов на Rust.

**Описание:** Yew предоставляет компонентную модель, состояние, обработку
событий и рендеринг в DOM. В проекте используется функциональный стиль
компонентов.

| Ресурс | Ссылка |
|---|---|
| Официальная документация | https://yew.rs/docs/ |
| Руководство (tutorial) | https://yew.rs/docs/next/tutorial/ |
| API на docs.rs | https://docs.rs/yew |

## 2. Trunk

**Назначение:** сборка и запуск UI‑проекта (WASM + статические файлы).

**Описание:** Trunk компилирует Rust в WebAssembly, подготавливает ресурсы
и предоставляет dev‑сервер.

| Ресурс | Ссылка |
|---|---|
| Официальный гайд | https://trunkrs.dev/ |

## 3. WebAssembly и wasm‑bindgen

**Назначение:** запуск Rust‑кода в браузере и обмен данными с JavaScript.

**Описание:** WebAssembly — формат исполняемого кода, а `wasm‑bindgen`
позволяет безопасно связывать Rust и Web API.

| Ресурс | Ссылка |
|---|---|
| wasm‑bindgen guide | https://rustwasm.github.io/docs/wasm-bindgen/ |
| MDN WebAssembly | https://developer.mozilla.org/docs/WebAssembly |

## 4. web‑sys и js‑sys

**Назначение:** доступ к стандартным объектам браузера и JavaScript.

**Описание:** `web‑sys` предоставляет привязки к Web API (DOM, fetch и т.п.),
`js‑sys` — к стандартным объектам JavaScript.

| Ресурс | Ссылка |
|---|---|
| web‑sys | https://docs.rs/web-sys |
| js‑sys | https://docs.rs/js-sys |

## 5. gloo (gloo‑net, gloo‑storage)

**Назначение:** удобные обёртки для браузерных API.

**Описание:** `gloo‑net` используется для HTTP‑запросов, `gloo‑storage` — для
работы с `localStorage`.

| Ресурс | Ссылка |
|---|---|
| gloo | https://gloo-rs.web.app/ |
| gloo‑net | https://docs.rs/gloo-net |
| gloo‑storage | https://docs.rs/gloo-storage |

## 6. Serde и serde_json

**Назначение:** сериализация и десериализация данных в JSON.

**Описание:** Serde позволяет преобразовывать структуры Rust в JSON и обратно,
что необходимо для обмена данными с API.

| Ресурс | Ссылка |
|---|---|
| Serde | https://serde.rs/ |
| serde_json | https://docs.rs/serde_json |

## 7. async‑trait и thiserror

**Назначение:** удобство при описании асинхронных трейтов и ошибок.

**Описание:** `async‑trait` позволяет писать async‑методы в трейтах, а
`thiserror` упрощает создание собственных типов ошибок.

| Ресурс | Ссылка |
|---|---|
| async‑trait | https://docs.rs/async-trait |
| thiserror | https://docs.rs/thiserror |

## 8. wasm‑bindgen‑futures

**Назначение:** запуск асинхронных задач в браузере.

**Описание:** библиотека связывает Rust‑`Future` с JS‑`Promise` и даёт
`spawn_local` для запуска задач.

| Ресурс | Ссылка |
|---|---|
| wasm‑bindgen‑futures | https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/ |

## 9. Ссылки на документацию проекта

- `docs/stack_guide.md` — подробный обзор стека;
- `docs/yew_ui_guide.md` — учебный гайд по компонентам и UI;
- `docs/common_issues.md` — частые ошибки и их причины.

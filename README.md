# rust-gigachat-webapp

Учебное веб-приложение (UI/UX) на Rust, использующее API сервиса
`rust-gigachat-app`. Проект предназначен для студентов 1 курса (2 семестр)
в рамках направления ПОО.

Статус: подготовлена базовая структура проекта. Исходники интерфейса пока
созданы и готовы к запуску.

## Цели

- Показать практику разработки Web UI на Rust.
- Показать разделение фронтенда и бэкенда через HTTP API.
- Дать основу для лабораторных работ и экспериментов с UI/UX.

## Бэкенд API

Используются эндпоинты текущего проекта:
- GET /
- GET /health
- POST /ask

Базовый URL должен быть конфигурируемым (например, через переменную окружения).

## Выбранный фреймворк (обоснование)

- Основной выбор: Yew (зрелый и стабильный фронтенд-фреймворк на Rust).
- Альтернативы для справки: Dioxus, Leptos.

Подробности см. в docs/framework_selection.md.

## Запуск

### Требования

- Rust + wasm32-unknown-unknown target
- Trunk (`cargo install trunk`)

### Локальный запуск

```bash
# backend (в отдельном терминале)
cd /home/sorcerer/Projects/rust-gigachat-app
cargo run

# frontend
cd /home/sorcerer/Projects/rust-gigachat-webapp
trunk serve --open
```

По умолчанию UI обращается к `http://127.0.0.1:8000`. Базовый URL можно
переопределить:

- при сборке через `API_BASE_URL` (compile-time),
- в интерфейсе (значение сохраняется в localStorage).

### Сборка и затравочная проверка

Подробная учебная инструкция: `docs/build_and_run.md`.

### Технологический стек

Подробный учебный гайд по фреймворкам и библиотекам: `docs/stack_guide.md`.

Коротко:

```bash
NO_COLOR=true trunk build
NO_COLOR=true trunk serve --address 127.0.0.1 --port 8080
```

## Архитектура (DDD)

- `src/domain` — сущности и value objects (Question, ApiBaseUrl).
- `src/application` — порты и use-cases (AskQuestion, CheckHealth).
- `src/infrastructure` — HTTP-клиент к `rust-gigachat-app`.
- `src/app.rs` — UI-композиция на Yew.

## Следующие шаги

1. Утвердить минимальные UX-сценарии.
2. Описать экраны и компоненты.
3. Зафиксировать контракт API и обработку ошибок.
4. Подготовить набор заданий для студентов.

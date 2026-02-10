# rust-gigachat-webapp

Учебное веб-приложение (UI/UX) на Rust, использующее API сервиса
`rust-gigachat-app`. Проект предназначен для студентов 1 курса (2 семестр)
в рамках направления ПОО.

Статус: подготовлена базовая структура проекта. Интерфейс собирается и
запускается, подробности — в `docs/build_and_run.md`.

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

**Важно:** UI **не обращается к GigaChat напрямую**. Все запросы идут только к
backend‑API проекта `rust-gigachat-app`, который запускается отдельным процессом.

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
# замените /path/to на путь к вашему проекту
cd /path/to/rust-gigachat-app
cargo run

# frontend
cd /path/to/rust-gigachat-webapp
NO_COLOR=true trunk serve --open
```

По умолчанию UI обращается к `http://127.0.0.1:8000`. Базовый URL можно
переопределить:

- при сборке через `API_BASE_URL` (compile-time),
- в интерфейсе (значение сохраняется в localStorage).

Если запросы блокируются браузером, это может быть CORS. См. `docs/common_issues.md`
и `lab_materials/lab_work.md`.

### Сборка и затравочная проверка

Подробная учебная инструкция: `docs/build_and_run.md`.

### Технологический стек

Подробный учебный гайд по фреймворкам и библиотекам: `docs/stack_guide.md`.

Коротко:

```bash
NO_COLOR=true trunk build
NO_COLOR=true trunk serve --address 127.0.0.1 --port 8080
```

## Учебные материалы

Полный комплект учебных материалов размещён в `lab_materials/README.md`.

Рекомендуемый минимум:
- `lab_materials/lab_work.md` — лабораторная работа (теория + шаги);
- `docs/yew_ui_guide.md` — практический гайд по компонентам Yew;
- `docs/ui_ux_requirements.md` — требования к интерфейсу.

## Архитектура (DDD)

- `src/domain` — сущности и value objects (Question, ApiBaseUrl).
- `src/application` — порты и use-cases (AskQuestion, CheckHealth).
- `src/infrastructure` — HTTP-клиент к `rust-gigachat-app`.
- `src/app.rs` — UI-композиция на Yew.

## Git и beads (учебный workflow)

В проекте используется встроенный issue‑трекер **beads**. Рекомендуемый режим:

```bash
bd init
bd hooks install
git config merge.beads.driver true
```

Задачи создаются с кратким описанием (`--description`), а синхронизация с git
обычно выполняется автоматически через хуки. При сбоях используйте `bd sync`.

Подробности: `lab_materials/beads_guide.md`, `lab_materials/git_version_control.md`,
`lab_materials/git_github_setup.md`.

## Безопасность и публикация учебных материалов

Перед отправкой проекта студентам:

- **Не коммитьте секреты** (`.env`, токены, приватные ключи).
- **Проверьте историю git** на наличие секретов:
  ```bash
  git log -p --all | grep -i "api-key\\|token\\|secret"
  ```
- Убедитесь, что `.gitignore` исключает временные и сборочные каталоги
  (например, `/target` и `/dist`).

## Следующие шаги

1. Утвердить минимальные UX-сценарии.
2. Описать экраны и компоненты.
3. Зафиксировать контракт API и обработку ошибок.
4. Подготовить набор заданий для студентов.

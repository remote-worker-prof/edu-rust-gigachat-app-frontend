# Agent Instructions for rust-gigachat-webapp

## Project Overview

Учебное веб-приложение с UI/UX на Rust, использующее API проекта
`rust-gigachat-app`. Это отдельный проект, предназначенный для студентов.

## Key Constraints

- Приложение работает только как клиент к существующему API.
- Базовый URL API должен быть конфигурируемым.
- Секреты не хранятся в репозитории.

## Framework Choice

Основной фреймворк: Yew (зрелый фронтенд для Web UI на Rust).
Альтернативы упоминаются в документации, но не используются без решения
преподавателя.

## Development Notes

- Интерфейс должен быть доступным и понятным для студентов.
- Дизайн должен быть прагматичным, без излишних визуальных эффектов.
- Все изменения должны учитывать учебную цель и ясность кода.

## Documentation First

Перед внесением изменений смотреть:
- docs/framework_selection.md
- docs/ui_ux_requirements.md
- docs/api_contract.md

## Testing Expectations

При появлении исходников предусмотреть:
- базовые UI-тесты,
- проверку корректности запросов к API,
- обработку сетевых ошибок.

## Agent Notes (non-student)

- Вводный файл для Moodle должен иметь заголовок, который явно содержит номер лабораторной работы и её тему.

## Beads + Git Workflow

Проект использует **beads** как встроенный issue‑трекер.

Рекомендуемый режим (по документации beads):
- авто‑синхронизация + git‑хуки + merge‑driver для `.beads/issues.jsonl`.

Если это свежий клон:
```bash
bd init
bd hooks install
git config merge.beads.driver true
```

Используйте `bd sync` только как fallback, если auto‑sync/хуки не сработали.

Полезные команды:
- `bd ready` — найти доступную работу
- `bd create "Заголовок" --type task --priority 2 --description "что и зачем"` — создать задачу
- `bd update <id> --status in_progress` — взять задачу
- `bd close <id>` — закрыть задачу

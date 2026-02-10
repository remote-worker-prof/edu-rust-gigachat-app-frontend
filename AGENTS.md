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

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

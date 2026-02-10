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

### Обязательный порядок работы (симуляция живого разработчика)

**Строго обязательно (без исключений):**

1. **До начала любой работы:** выполнить `bd create` с описанием (`--description`).
2. **Перед изменениями:** перевести задачу в `in_progress`.
3. **Только после этого:** выполнять правки, тесты и исправления.
4. **Подготовить индекс:** добавить изменения в индекс (`git add -A`).
5. **Коммит проекта:** сообщение коммита должно начинаться с заголовка issue,
   а тело коммита должно совпадать с `--description`. После описания
   добавьте список изменённых файлов (см. шаблон ниже).
6. **Закрытие задачи:** `bd close <id>` выполняется после коммита проекта.
7. **Синхронизация beads:** `bd sync` выполняется после `bd close`.

**Важно:** `bd sync` коммитит **только** `.beads/issues.jsonl`
(данные beads). Изменения проекта он **не коммитит**.
Коммит проекта выполняется вручную. `bd sync` не выполняет
`git add`, поэтому новые файлы нужно добавить в индекс заранее.
Повторный push допускается только если `bd sync` не выполнялся
или завершился с ошибкой.

**Шаблон коммита (issue → commit):**
```bash
TITLE="<issue title>"
DESC="<issue description>"
FILES=$(git diff --cached --name-only | sed 's/^/- /')
printf "%s\n\n%s\n\nИзменения:\n%s\n" "$TITLE" "$DESC" "$FILES" | git commit -F -
```

Вариант со статусами файлов (A/M/D/R/C + пояснение):
```bash
TITLE="<issue title>"
DESC="<issue description>"
FILES=$(git diff --cached --name-status | awk '
  {
    code=$1; from=$2; to=$3;
    status="изменён";
    if (code ~ /^A/) status="создан";
    else if (code ~ /^D/) status="удалён";
    else if (code ~ /^R/) { status="переименован"; printf "- %s %s -> %s (%s)\n", code, from, to, status; next }
    else if (code ~ /^C/) { status="скопирован"; printf "- %s %s -> %s (%s)\n", code, from, to, status; next }
    printf "- %s %s (%s)\n", code, from, status;
  }')
printf "%s\n\n%s\n\nИзменения:\n%s\n" "$TITLE" "$DESC" "$FILES" | git commit -F -
```

Если `.beads/issues.jsonl` попал в индекс, его можно исключить из проектного
коммита и оставить для `bd sync`:
```bash
git restore --staged .beads/issues.jsonl
```

**Примечание про хуки:** если `bd sync` вызывается внутри git‑хуков,
он может запускать повторную синхронизацию. Это ожидаемое поведение,
и ручной `git push` после него не требуется.

**Формальное описание процесса (CNCF):**
- `agents-issue-workflow.cncf.yaml` — Serverless Workflow (CNCF) для агентских правил.

**Запрещено:**
- начинать работу без `bd create`;
- закрывать задачу до `bd sync`;
- выполнять работу, если задача не в `in_progress`.

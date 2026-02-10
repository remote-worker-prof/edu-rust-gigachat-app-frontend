# Сборка и запуск UI

## Требования

- Rust (stable)
- Цель компиляции WebAssembly: `wasm32-unknown-unknown`
- Trunk (инструмент сборки Yew)

Установка:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Запуск (режим разработки)

1. Запустите backend (в отдельном терминале):

```bash
cd /home/sorcerer/Projects/rust-gigachat-app
cargo run
```

2. Запустите UI:

```bash
cd /home/sorcerer/Projects/rust-gigachat-webapp
NO_COLOR=true trunk serve --address 127.0.0.1 --port 8080
```

Откройте в браузере: `http://127.0.0.1:8080`.

> Примечание: если в окружении задан `NO_COLOR=1`, Trunk может падать с
> ошибкой `invalid value '1' for '--no-color'`. Используйте `NO_COLOR=true`.

## Сборка

```bash
NO_COLOR=true trunk build
```

## Затравочная проверка

Минимальная проверка, что UI собирается и отвечает:

```bash
NO_COLOR=true trunk serve --address 127.0.0.1 --port 8080
```

Затем откройте `http://127.0.0.1:8080` и убедитесь, что загрузился экран
с формой вопроса и блоком статуса API.

## Настройка API_BASE_URL

- По умолчанию UI обращается к `http://127.0.0.1:8000`.
- Можно задать compile-time переменную:

```bash
API_BASE_URL=http://127.0.0.1:8000 NO_COLOR=true trunk serve --address 127.0.0.1 --port 8080
```

- Или изменить базовый URL в интерфейсе (значение хранится в localStorage).

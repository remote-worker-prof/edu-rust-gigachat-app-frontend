# Лабораторная работа №3 (Frontend): развёртывание edu-rust-gigachat-app-frontend в Yandex Cloud

Документ предназначен для самостоятельной практики. Текст ориентирован на
начинающих студентов и описывает полный путь: подготовка аккаунта, контейнеризация
UI, загрузка образа в Yandex Container Registry и запуск в Yandex Cloud Serverless
Containers.

## 1. Цель работы

1. Развернуть учебное веб‑приложение `edu-rust-gigachat-app-frontend` как
   serverless‑контейнер.
2. Хранить Docker‑образ в Yandex Container Registry.
3. Работать с Yandex Cloud через `yc` и Docker, используя долгоживущий токен.
4. Получить публичный URL контейнера и проверить работу UI.

## 2. Что нужно заранее

- Аккаунт в Yandex Cloud с доступом к консоли.
- Установленный `yc` CLI.
- Установленный Docker.
- Доступ к репозиторию `edu-rust-gigachat-app-frontend`.

### 2.1. Установка Docker (по ОС)

Вариант зависит от операционной системы. Используйте официальные инструкции.

**Windows**
- **Docker Desktop (рекомендуемый вариант для учебной работы).**
  Официальная инструкция: https://docs.docker.com/desktop/setup/install/windows-install/
- **Docker Desktop + WSL 2.**
  Для работы на Windows требуется WSL 2. Установка WSL: https://learn.microsoft.com/en-us/windows/wsl/install
  Дополнительно: раздел про WSL в Docker Desktop: https://docs.docker.com/desktop/features/wsl/use-wsl/

**macOS**
- **Docker Desktop for Mac.**
  Официальная инструкция: https://docs.docker.com/installation/mac/

**Linux**
- **Docker Desktop for Linux** (если нужен GUI‑инструмент):
  https://docs.docker.com/desktop/setup/install/linux/
- **Docker Engine** (классический вариант для Linux):
  https://docs.docker.com/engine/install/

## 3. Термины и токены

### 3.1. OAuth‑токен (долгоживущий)

OAuth‑токен используется для работы `yc` CLI и для доступа к Container Registry.
Его стандартный срок действия — **12 месяцев**, поэтому он подходит для
«годичного» сценария, указанного в задании.

**Важно:** токен — это секрет. Его нельзя добавлять в git.

### 3.2. IAM‑токен (короткоживущий)

IAM‑токен нужен для прямых обращений к API Yandex Cloud и для проверочных
запросов. Максимальный срок действия — **12 часов**.

### 3.3. API‑ключ сервисного аккаунта (альтернатива)

Для автоматизации можно создать API‑ключ сервисного аккаунта с **любым сроком
действия**, например на 1 год. Это альтернатива OAuth‑токену и удобна, когда
работа выполняется без личного аккаунта.

## 4. Подготовка `yc` CLI

1. Выполнить начальную настройку `yc` CLI (авторизация через OAuth‑токен).
2. Задать каталог по умолчанию:

```bash
yc config set folder-id <FOLDER_ID>
```

## 4.1. Локальная dev‑сборка UI через Trunk (кратко)

Для подготовки статических файлов нужен Trunk и целевая платформа WebAssembly.
Если они ещё не установлены:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

**Почему target называется `wasm32-unknown-unknown`:**
- `wasm32` — архитектура WebAssembly 32‑bit;
- первое `unknown` — нет конкретной ОС (код исполняется в браузере);
- второе `unknown` — нет стандартного ABI, взаимодействие идёт через Web API.
Для Yew + Trunk в браузере используется именно этот target.

Минимальный dev‑запуск:

```bash
NO_COLOR=true trunk serve --address 127.0.0.1 --port 8080
```

Подробная учебная инструкция: `docs/build_and_run.md`.

## 5. Создание Container Registry

Создать реестр:

```bash
yc container registry create --name <registry-name>
```

Проверить список реестров и получить `REGISTRY_ID`:

```bash
yc container registry list
```

## 6. Настройка Docker для Yandex Registry

Подключить credential helper:

```bash
yc container registry configure-docker
```

Команда должна выполняться **без `sudo`**, иначе Docker не найдёт helper.

## 7. Сборка UI и Docker‑образа

### 7.1. Сборка статических файлов

Для Docker‑образа нужен **статический билд**, поэтому используется `trunk build`
(не `trunk serve`).

```bash
cd /ваш/путь/к/edu-rust-gigachat-app-frontend
NO_COLOR=true trunk build
```

После сборки статические файлы появятся в `dist/`.

### 7.2. Dockerfile (подробный учебный образец)

Serverless Containers ожидают, что приложение слушает порт из переменной
окружения `PORT`. Ниже приведён **полный Dockerfile**, который:
- использует лёгкий базовый образ `nginx:alpine`;
- копирует статические файлы из `dist/`;
- подставляет порт из переменной окружения `PORT`;
- корректно запускает nginx в foreground‑режиме.

```Dockerfile
# 1. Базовый образ: минимальный nginx на Alpine Linux.
FROM nginx:alpine

# 2. Устанавливаем bash и envsubst (часть пакета gettext),
#    чтобы подставлять значение $PORT в шаблон конфигурации.
RUN apk add --no-cache bash gettext

# 3. Копируем статические файлы, собранные trunk build.
#    Это готовый фронтенд (HTML/CSS/JS/WASM).
COPY dist/ /usr/share/nginx/html/

# 4. Копируем шаблон конфигурации nginx, где порт указан как ${PORT}.
COPY nginx.conf.template /etc/nginx/templates/default.conf.template

# 5. Значение PORT по умолчанию (может быть переопределено в Yandex Cloud).
ENV PORT=8080

# 6. При старте контейнера подставляем PORT и запускаем nginx.
#    Важно: nginx должен работать в foreground‑режиме (daemon off),
#    иначе контейнер сразу завершится.
CMD ["/bin/sh", "-c", "envsubst '$$PORT' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"]
```

#### Подробные пояснения к Dockerfile (в текстовом виде)

1. **FROM nginx:alpine** — берём официальный минимальный nginx. Он маленький,
   быстро скачивается и достаточно для раздачи статических файлов.
2. **RUN apk add --no-cache bash gettext** — добавляем `bash` и `envsubst`.
   Пояснение для новичков:
   - `apk` — менеджер пакетов Alpine Linux (аналог `apt`/`yum`);
   - `bash` — оболочка, из которой запускаем `envsubst`;
   - `gettext` — пакет, внутри которого находится утилита `envsubst`;
   - `envsubst` заменяет `${PORT}` в шаблоне nginx на реальное значение порта.
3. **COPY dist/** — переносим результат `trunk build` в каталог nginx
   `/usr/share/nginx/html`. Это стандартное место для статических файлов.
4. **COPY nginx.conf.template** — кладём шаблон конфигурации nginx, в котором
   порт задан как переменная `${PORT}`, а не жёстким числом.
5. **ENV PORT=8080** — значение по умолчанию. В Yandex Cloud эта переменная
   задаётся автоматически (или вручную в параметрах ревизии).
6. **CMD ... envsubst ... nginx -g 'daemon off;'** — при старте контейнера
   подставляем порт и запускаем nginx в foreground‑режиме. Это обязательное
   требование контейнерной платформы: основной процесс не должен «уходить в фон».

**Файлы в репозитории:**
- `Dockerfile` — полный учебный Dockerfile из примера выше (в корне проекта).
- `nginx.conf.template` — шаблон конфигурации nginx (в корне проекта).

Пример `nginx.conf.template`:

```nginx
server {
  listen ${PORT};
  server_name _;

  root /usr/share/nginx/html;
  index index.html;

  location / {
    try_files $uri /index.html;
  }
}
```

### 7.3. Зачем нужен nginx и можно ли без него

**Зачем нужен nginx в нашем Docker‑образе:**

1. **Нужен HTTP‑сервер.** Serverless Containers запускает контейнер и
   ожидает, что внутри будет процесс, который **слушает HTTP‑порт** (`PORT`)
   и отдаёт ответы. Статические файлы сами по себе не «работают» без сервера.
2. **Раздача SPA.** Наш UI — это single‑page‑application. Поэтому любая
   страница должна отдавать `index.html`, а это обеспечивается правилом
   `try_files $uri /index.html;` в конфигурации nginx.
3. **Стандартный и надёжный инструмент.** nginx компактный, стабильный и
   широко используется для раздачи статических файлов в продакшене.

**Насколько nginx обязателен?**

Технически **не обязателен**. В контейнере можно использовать любой HTTP‑сервер,
который умеет:
- слушать порт из переменной окружения `PORT`;
- отдавать файлы из `dist/`;
- корректно обслуживать SPA‑маршруты (аналог `try_files /index.html`).

Примеры альтернатив: `caddy`, `busybox httpd`, `python -m http.server`
(последний — только для учебных целей, не для продакшена).

**Почему в учебной работе выбран nginx:**

Он минимально усложняет настройку, совпадает с индустриальной практикой и
обеспечивает корректную работу SPA‑маршрутов без дополнительного кода.

## 8. Сборка и отправка образа в Registry

1. Собрать образ:

```bash
docker build -t cr.yandex/<REGISTRY_ID>/edu-rust-gigachat-app-frontend:latest .
```

2. Отправить образ:

```bash
docker push cr.yandex/<REGISTRY_ID>/edu-rust-gigachat-app-frontend:latest
```

## 9. Создание Serverless Container

Создать контейнер:

```bash
yc serverless container create --name <container-name>
```

CLI вернёт URL контейнера — он понадобится для проверки.

## 10. Развёртывание ревизии

Создать ревизию контейнера с указанием образа:

```bash
yc serverless container revision deploy \
  --container-name <container-name> \
  --image cr.yandex/<REGISTRY_ID>/edu-rust-gigachat-app-frontend:latest \
  --cores 1 \
  --memory 512M \
  --execution-timeout 30s
```

Если registry приватный, контейнеру нужен сервисный аккаунт с ролью
`container-registry.images.puller`, а у пользователя должна быть роль
`iam.serviceAccounts.user`, чтобы использовать этот сервисный аккаунт.

## 11. Переменные окружения для UI

Если нужно зафиксировать адрес backend‑API, добавьте переменную `API_BASE_URL`
на уровне ревизии контейнера. **Добавление переменных окружения создаёт
новую ревизию**.

## 12. Проверка запуска

По умолчанию контейнер защищён: для вызова нужен IAM‑токен.
Пример проверки:

```bash
curl -H "Authorization: Bearer $(yc iam create-token)" https://<container-url>
```

## 13. Результаты, которые нужно показать преподавателю

1. URL serverless‑контейнера.
2. Список ревизий контейнера.
3. Docker‑образ в Yandex Container Registry.
4. Скриншот работающего UI.

## 14. Официальные источники

- Serverless Containers (обзор): https://yandex.cloud/en/docs/serverless-containers/concepts/container
- Serverless Containers (quickstart/CLI): https://yandex.cloud/en/docs/serverless-containers/quickstart
- Переменные окружения в контейнерах: https://yandex.cloud/en/docs/serverless-containers/operations/environment-variables
- Container Registry: https://yandex.cloud/en/docs/container-registry/
- Настройка Docker: https://yandex.cloud/en/docs/container-registry/operations/authentication
- OAuth‑токены: https://yandex.cloud/en/docs/iam/concepts/authorization/oauth-token
- IAM‑токены: https://yandex.cloud/en/docs/iam/concepts/authorization/iam-token
- API‑ключи сервисных аккаунтов: https://yandex.cloud/en/docs/security/standards/authentication

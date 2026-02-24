# Практическая работа: развёртывание edu-rust-gigachat-app-frontend как Serverless Application в Yandex Cloud

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

```bash
cd /ваш/путь/к/edu-rust-gigachat-app-frontend
trunk build
```

После сборки статические файлы появятся в `dist/`.

### 7.2. Минимальный Dockerfile

Serverless Containers ожидают, что приложение слушает порт из переменной
окружения `PORT`. Ниже приведён минимальный пример контейнера, который
раздаёт статические файлы через nginx и использует `PORT`.

```Dockerfile
FROM nginx:alpine

# Подставляем порт из переменной окружения PORT
RUN apk add --no-cache bash

COPY dist/ /usr/share/nginx/html/
COPY nginx.conf.template /etc/nginx/templates/default.conf.template

# Nginx сам подставит PORT при запуске через envsubst
ENV PORT=8080

CMD ["/bin/sh", "-c", "envsubst '$$PORT' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"]
```

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

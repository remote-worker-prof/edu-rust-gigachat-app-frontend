# 1. Базовый образ: минимальный nginx на Alpine Linux.
FROM nginx:alpine

# 2. Устанавливаем bash и envsubst (часть пакета gettext),
#    чтобы подставлять значение $PORT в шаблон конфигурации.
#    Пояснение для новичков:
#    - apk — менеджер пакетов Alpine Linux (аналог apt/yum), через него ставим утилиты.
#    - bash — удобный shell, из которого мы запускаем команду envsubst при старте.
#    - gettext включает утилиту envsubst, которая подставляет переменные окружения
#      в текст (в нашем случае ${PORT} внутри шаблона nginx).
#    - значение PORT приходит из окружения (платформа Serverless Containers)
#      или берётся из ENV PORT ниже, если его не передали.
RUN apk add --no-cache bash gettext

# 3. Копируем статические файлы, собранные trunk build.
#    Это готовый фронтенд (HTML/CSS/JS/WASM).
COPY dist/ /usr/share/nginx/html/

# 4. Копируем шаблон конфигурации nginx, где порт указан как ${PORT}.
COPY nginx.conf.template /etc/nginx/templates/default.conf.template

# 5. Значение PORT по умолчанию (может быть переопределено в Yandex Cloud).
#    В Serverless Containers платформа передаёт PORT как переменную окружения.
#    Если не передали вручную, используем 8080.
ENV PORT=8080

# 6. При старте контейнера подставляем PORT и запускаем nginx.
#    Важно: nginx должен работать в foreground‑режиме (daemon off),
#    иначе контейнер сразу завершится.
CMD ["/bin/sh", "-c", "envsubst '$$PORT' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"]

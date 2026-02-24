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

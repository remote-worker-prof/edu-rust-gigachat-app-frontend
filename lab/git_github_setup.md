# Практикум: SSH‑ключи и подключение локального репозитория к GitHub

Документ предназначен для студентов и описывает современный, воспроизводимый порядок:
1) создать SSH‑ключ (Ed25519),
2) добавить ключ в GitHub,
3) создать пустой репозиторий на GitHub,
4) подключить к нему уже существующий локальный репозиторий через `git remote`.

Основано на официальной документации GitHub и Git.

---

## 1. Создание SSH‑ключа (Ed25519)

**Рекомендуемый алгоритм:** Ed25519 — современный эллиптический алгоритм ключей для SSH.

Откройте терминал и выполните:

```bash
ssh-keygen -t ed25519 -C "you@example.com"
```

Что важно:
- если ключи уже существуют, безопаснее создать новый с отдельным именем
  (например, `id_ed25519_github`), чтобы не перезаписывать старые;
- при запросе пароля (passphrase) задайте его — это рекомендуемая практика безопасности.

---

## 2. Добавление ключа в ssh‑agent

### Linux / WSL

```bash
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
```

Если ключ называется иначе — укажите свой файл.

### Почему пароль может запрашиваться снова

`ssh-agent` хранит ключи **только в памяти процесса**. Если при каждом открытии
терминала запускается новый агент, то ключи «теряются», и пароль требуется снова.
Это нормально, но неудобно.

### Как сделать так, чтобы пароль вводился один раз за сессию WSL

Ниже — простой и устойчивый вариант для `bash`: один агент на сессию, а ключ
добавляется только если он ещё не добавлен.

Добавьте в `~/.bashrc`:

```bash
# ----- ssh-agent (persistent across terminals) -----
SSH_KEY="$HOME/.ssh/id_ed25519"
SSH_AGENT_ENV="$HOME/.ssh/agent.env"

load_ssh_agent() {
  if [ -f "$SSH_AGENT_ENV" ]; then
    # shellcheck disable=SC1090
    . "$SSH_AGENT_ENV" >/dev/null 2>&1
  fi

  if [ -n "$SSH_AUTH_SOCK" ] && [ -S "$SSH_AUTH_SOCK" ]; then
    return 0
  fi

  eval "$(ssh-agent -s)" >/dev/null
  umask 077
  {
    echo "export SSH_AUTH_SOCK=$SSH_AUTH_SOCK"
    echo "export SSH_AGENT_PID=$SSH_AGENT_PID"
  } > "$SSH_AGENT_ENV"
}

load_ssh_agent

if [ -t 1 ] && [ -f "$SSH_KEY" ]; then
  if ! ssh-add -l >/dev/null 2>&1; then
    ssh-add "$SSH_KEY" >/dev/null 2>&1 || true
  fi
fi
```

После этого:
- пароль вводится один раз после перезапуска WSL,
- в новых терминалах ключ уже доступен без повторного ввода.

### Windows (PowerShell, OpenSSH)

```powershell
Get-Service -Name ssh-agent | Set-Service -StartupType Manual
Start-Service ssh-agent
ssh-add $env:USERPROFILE\.ssh\id_ed25519
```

Если ключ называется иначе — укажите свой файл.

---

## 3. Добавление ключа в `~/.ssh/config`

Это повышает стабильность работы и устраняет неоднозначность выбора ключа.

### Windows

Файл: `C:\Users\<USERNAME>\.ssh\config` (или `~/.ssh/config`).

```sshconfig
Host github.com
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519
  IdentitiesOnly yes
```

### Linux / WSL

Файл: `~/.ssh/config`.

```sshconfig
Host github.com
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519
  IdentitiesOnly yes
```

Шаблон конфигурации основан на официальной практике GitHub для `~/.ssh/config`.

---

## 4. Добавление публичного ключа в GitHub

Сначала скопируйте публичный ключ:

**Linux / WSL:**
```bash
cat ~/.ssh/id_ed25519.pub
```

**WSL (копирование в буфер Windows):**
```bash
clip.exe < ~/.ssh/id_ed25519.pub
```

Если `clip.exe` не работает в вашей bash‑сессии, просто выведите ключ в терминал:
```bash
cat ~/.ssh/id_ed25519.pub
```

**Windows (PowerShell):**
```powershell
clip < $env:USERPROFILE\.ssh\id_ed25519.pub
```

GitHub официально рекомендует использовать `clip.exe` в WSL.

Далее:
1. Откройте **Settings → SSH and GPG keys → New SSH key**.
2. Вставьте публичный ключ и сохраните.

---

## 5. Проверка SSH‑подключения

```bash
ssh -T git@github.com
```

В ответе должно быть:
`Hi <username>! You've successfully authenticated...`

---

## 6. Создание пустого репозитория на GitHub

Создайте новый репозиторий через веб‑интерфейс GitHub и **не**
инициализируйте его README, `.gitignore` или лицензией — это важно
при подключении уже существующего локального репозитория.

---

## 7. Подключение существующего локального репозитория

Перейдите в каталог проекта и добавьте remote:

```bash
git remote add origin git@github.com:USERNAME/REPOSITORY.git
```

Проверьте:
```bash
git remote -v
```

Эти шаги соответствуют официальной инструкции GitHub по подключению
локального репозитория.

Если remote `origin` уже существует:
```bash
git remote set-url origin git@github.com:USERNAME/REPOSITORY.git
```

Команда `set-url` описана в официальной документации Git.

---

## 8. Первый push в GitHub

```bash
git push -u origin main
```

Если ваша ветка называется иначе, замените `main` на фактическое имя ветки.

---

## 9. Мини‑шпаргалка (краткая версия)

```bash
ssh-keygen -t ed25519 -C "you@example.com"
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
clip.exe < ~/.ssh/id_ed25519.pub   # WSL, копировать ключ

git remote add origin git@github.com:USERNAME/REPOSITORY.git
git push -u origin main
```

Команды повторяют ключевые шаги из официальной инструкции GitHub по
подключению локального репозитория.

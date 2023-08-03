# Local Status Boost

Local Status Boost - это программа, которая автоматически репостит посты в локальной ленте Mastodon, игнорируя определенный аккаунт.

### Требования

- Rust 1.71.0 (возможны и более старые версии, но не тестировал)
- Доступ к сети Mastodon и соответствующий access token

### Установка

1. Клонируйте репозиторий

```
git clone https://github.com/yourusername/local-status-boost.git
```

2. Запустите Cargo для сборки проекта

```
cargo build --release
```

### Настройка

Программа использует файл `Config.toml` для своих настроек. Он должен быть настроен следующим образом:

```toml
api_host = "https://your-instance-here"
access_token = "your-access-token-here"
filter_account = "@account-to-ignore"
```

Замените `https://your-instance-here` на URL вашего сервера Mastodon, `your-access-token-here` на ваш access token, а `@account-to-ignore` на аккаунт, который вы хотите игнорировать (рекомендуется использовать @local и он естественно должен совпадать с тем от имени которого вы будете репостить посты).

### Запуск

Просто запустите скомпилированную программу:

```
./target/release/local-status-boost
```

Программа будет автоматически репостить все посты в локальной ленте, за исключением постов от аккаунта, указанного в `filter_account`.

### Завершение работы

Чтобы прервать программу, просто нажмите `Ctrl+C`.# local_status_boost
FROM rust:1.71.0 as builder
WORKDIR /usr/src/local_status_boost
COPY ./local_status_boost .
# Собираем приложение <-j$(nproc)> можно убрать, но возможно будет медленно
# или можно указать конкретное количество ядер -j4 например
RUN cargo build --release -j$(nproc)
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
FROM rust:1.71.0
WORKDIR /app
COPY --from=builder /usr/src/local_status_boost/target/release/local ./local
COPY ./local_status_boost/Config.toml ./Config.toml
RUN chmod +x ./local
CMD ["./local"]
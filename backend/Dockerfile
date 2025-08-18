# 1. ビルドステージ
FROM rust:1.84 as builder
WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . ./backend
WORKDIR /usr/src/app/backend
RUN cargo build --release

# 2. 実行ステージ
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# バイナリコピー
COPY --from=builder /usr/src/app/backend/target/release/backend /app/backend

# データコピー
COPY ./data /app/data

ENV PORT=3000
EXPOSE 3000
CMD ["/app/backend"]
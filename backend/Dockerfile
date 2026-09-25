FROM rust:1.84 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
COPY data/problems.json data/problem-models.json /app/data/

ENV PORT=3000
ENV DATA_DIR=/app/data

EXPOSE 3000

CMD ["/app/backend"]

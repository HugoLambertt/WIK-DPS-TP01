FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN useradd -ms /bin/bash user_tp
USER user_tp
WORKDIR /app

COPY --from=builder /app/target/release/srv-web .

EXPOSE 8080
ENV PING_LISTEN_PORT=8080

CMD ["./srv-web"]
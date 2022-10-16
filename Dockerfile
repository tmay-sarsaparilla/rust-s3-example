FROM rust:1.64.0-slim as builder

ENV APP_DIR=/usr/src/app

WORKDIR ${APP_DIR}
COPY src/ ./src
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN cargo install --path ${APP_DIR}

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/local/cargo/bin/rust_example .

EXPOSE 8080

CMD ["/rust_example"]

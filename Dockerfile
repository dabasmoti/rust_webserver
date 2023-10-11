# Use the official Rust image as the builder stage
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:stable-slim
RUN apt update &&  apt upgrade -y && apt install -y curl


WORKDIR /app
COPY --from=builder /app/target/release/ .
COPY --from=builder /app/predic.json/ .

CMD ["./rust_json_webserver"]

# Contains multistep process
# Build stage
FROM rust:1.76.0-slim-bookworm as builder
WORKDIR /app
RUN apt-get update -y && apt-get upgrade -y && apt install -y pkg-config libssl-dev
RUN mkdir src
RUN mkdir templates
COPY Cargo.toml .
Copy src/ ./src
COPY templates/ ./templates
RUN cargo build --release --bin account

# Prod stage
FROM debian:bookworm-20240211-slim
WORKDIR /app
EXPOSE 8080
RUN apt-get update -y && apt-get upgrade -y && apt install -y pkg-config libssl-dev ca-certificates
COPY --from=builder /app/target/release/account /app/account
CMD ["./account"]
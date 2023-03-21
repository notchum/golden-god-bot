# Build stage
FROM rust:latest as builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/golden-god-bot /app
COPY --from=builder /app/json/ /app/json/
CMD ["./golden-god-bot"]
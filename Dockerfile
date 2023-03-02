FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/golden-god-bot"]
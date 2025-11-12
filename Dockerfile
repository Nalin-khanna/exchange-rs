
FROM rust:1.88-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

COPY . .

RUN cargo build --release

# use a minimal Debian slim image for a small final container
FROM debian:12-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ARG JWT_SECRET
ARG PORT=8080 

ENV JWT_SECRET=${JWT_SECRET}
ENV PORT=${PORT}

# Copy only the compiled binary from the builder stage
COPY --from=builder /app/target/release/exchange_rs ./exchange_rs

EXPOSE 8080

CMD ["./exchange_rs"]
FROM rust:1.85-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN mkdir -p /app/data

COPY --from=builder /app/target/release/icnvb-backend .

ENV PORT=3000
ENV DATA_DIR=/app/data
ENV ALLOWED_ORIGINS="https://www.icnvb.vercel.app,http://localhost:5173"

EXPOSE 3000

CMD ["./icnvb-backend"]

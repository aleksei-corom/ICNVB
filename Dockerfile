FROM rust:1-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copiar archivos fuente del backend (funciona tanto si el contexto es la raíz como si es backend/)
COPY Cargo.tom[l] backend/Cargo.tom[l] ./
COPY sr[c] backend/sr[c] ./src/

# Compilar release directamente (Cargo genera un lockfile limpio compatible con Linux)
RUN cargo build --release

# ============================================================================
# Etapa 2: Imagen final de producción limpia y ligera
# ============================================================================
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN mkdir -p /app/data /app/uploads

COPY --from=builder /app/target/release/icnvb-backend .

ENV PORT=3000
ENV DATA_DIR=/app/data
ENV ALLOWED_ORIGINS="https://www.icnvb.vercel.app,https://icnvb.vercel.app,http://localhost:5173"

EXPOSE 3000

CMD ["./icnvb-backend"]

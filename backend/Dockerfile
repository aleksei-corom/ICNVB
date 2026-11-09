# ============================================================================
# Etapa 1: Build del backend en Rust
# ============================================================================
FROM rust:1.80-slim AS builder

WORKDIR /app

# Instalar dependencias del sistema necesarias para compilar
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar solo los archivos de dependencias primero (para cache de Docker)
COPY Cargo.toml Cargo.lock ./

# Crear un main.rs vacío para pre-compilar dependencias (cache de capas)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copiar el código fuente real
COPY . .

# Forzar recompilación con el código real
RUN touch src/main.rs && cargo build --release

# ============================================================================
# Etapa 2: Imagen de producción ligera
# ============================================================================
FROM debian:bookworm-slim

# Instalar solo librerías de runtime necesarias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Crear directorio para datos persistentes
RUN mkdir -p /app/data

# Copiar el binario compilado desde la etapa de build
COPY --from=builder /app/target/release/icnvb-backend .

# Variables de entorno por defecto
ENV PORT=3000
ENV DATA_DIR=/app/data
ENV ALLOWED_ORIGINS="https://www.icnvb.vercel.app,http://localhost:5173"

# Puerto que expone la aplicación
EXPOSE 3000

# Comando de inicio
CMD ["./icnvb-backend"]

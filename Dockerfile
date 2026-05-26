# ============================================================
# STAGE 1 — Builder
# ============================================================
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

# Instalar dependencias del sistema necesarias para sqlx / openssl
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar manifiestos primero para aprovechar la caché de capas
COPY Cargo.toml Cargo.lock ./

#cache offline dependencies
COPY .sqlx ./.sqlx

#Variable para trabajar con sqlx offline
ENV SQLX_OFFLINE=true

# Crear un src/main.rs vacío para pre-compilar dependencias
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/clinica_api*

# Copiar el código fuente real
COPY src ./src
COPY migrations ./migrations

# Compilar el binario final
RUN cargo build --release

# ============================================================
# STAGE 2 — Runtime (imagen mínima)
# ============================================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Instalar libssl en runtime (requerido por sqlx con TLS)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario sin privilegios
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    appuser

# Copiar el binario compilado desde el builder
COPY --from=builder /app/target/release/clinica_api /app/clinica_api

# Copiar migraciones para que sqlx-cli las ejecute si es necesario
COPY --from=builder /app/migrations /app/migrations

USER appuser

EXPOSE 3000

CMD ["/app/clinica_api"]
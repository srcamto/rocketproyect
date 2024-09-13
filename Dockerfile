# Build stage
FROM rust:1.72-buster AS builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Copiar el código fuente al contenedor de construcción
COPY . . 

# Compilar la aplicación en modo de liberación
RUN cargo build --release

# Production stage
FROM debian:stable-20240812-slim

WORKDIR /app

# Copiar el binario desde la etapa de construcción
COPY --from=builder /app/target/release/testRust .
EXPOSE 8000
# Comando para ejecutar la aplicación
CMD ["./testRust"]
FROM rust:1.76-slim

WORKDIR /app

# Instalar dependencias necesarias
RUN apt-get update && \
    apt-get install -y git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copiar archivos del proyecto
COPY . .

# Compilar el proyecto
RUN cargo build

# Configuración para ejecutar pruebas
ENV RUST_BACKTRACE=1

# Establecer el punto de entrada
ENTRYPOINT ["/app/target/debug/ignore"] 
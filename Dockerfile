# FRONTEND
FROM node:24-alpine3.22 AS frontend-builder

WORKDIR /workdir

COPY frontend/package*.json ./
RUN npm install

COPY frontend/ ./
RUN npm run build

# BACKEND
FROM rust:1.89-alpine3.22 AS backend-builder

RUN apk add --no-cache musl-dev

WORKDIR /workdir

# Create a dummy project so Docker caches dependencies
RUN cargo new --bin backend
WORKDIR /workdir/backend

# Copy Cargo.toml + Cargo.lock (if exists) first for dependency caching
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN cargo fetch
RUN cargo build --release

# Copy the actual backend source
COPY backend/src ./src

# Build release binary
RUN cargo build --release


# FINAL IMAGE
FROM alpine:3.22

ARG LOG_LEVEL="info"

WORKDIR /app

# Copy release binary from backend-builder
COPY --from=backend-builder /workdir/backend/target/release/yathzee-world .

# Copy frontend build (if you need it at runtime for Actix static serving)
COPY --from=frontend-builder /workdir/build ./static-frontend

# Expose port 8080
EXPOSE 8080

# Run the server
CMD ["/bin/sh", "-c", "RUST_LOG=${LOG_LEVEL} ./yathzee-world"]

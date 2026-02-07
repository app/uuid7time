# ---------- Stage 1: Build ----------
FROM rust:alpine AS builder
WORKDIR /app

# Install musl-dev for static linking
RUN apk add --no-cache musl-dev

# Cache dependencies to avoid recompiling them on every code change
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs   # placeholder
RUN cargo fetch   # download dependencies

# Copy actual source code and build static release binary
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# ---------- Stage 2: Minimal runtime ----------
FROM scratch AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/uuid7time /app/uuid7time
ENTRYPOINT ["/app/uuid7time"]

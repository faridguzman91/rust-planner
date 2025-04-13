# syntax=docker/dockerfile:1.4
ARG RUST_VERSION=1.71.0
ARG APP_NAME=ActixWebTaskService

# --- Build stage ---
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF


# --- Runtime stage ---
FROM debian:bullseye-slim AS final

# Install CA certificates for HTTPS (used by AWS SDKs etc.)
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user for running the app
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Switch to the unprivileged user
USER appuser

# Copy the built binary from the builder stage
COPY --from=build /bin/server /bin/

# Expose app port (adjust if needed)
EXPOSE 80

# Entrypoint
CMD ["/bin/server"]


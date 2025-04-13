# syntax=docker/dockerfile:1.4
ARG RUST_VERSION=1.81.0
ARG APP_NAME=rust-planner

# --- Build stage ---
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app


RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    bash -c "set -e && cargo build --locked --release && cp ./target/release/${APP_NAME} /bin/server"


ENV AWS_REGION=us-east-1
# ENV AWS_ACCESS_KEY_ID=your_access_key_id
# ENV AWS_SECRET_ACCESS_KEY=your_secret_access_key
# ENV AWS_REGION=us-east-1  # Replace with your region

# --- Runtime stage ---
FROM debian:bullseye-slim AS final

# Install CA certificates for HTTPS (used by AWS SDKs etc.)
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates && rm -rf /var/lib/apt/lists/*

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


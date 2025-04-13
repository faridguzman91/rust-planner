# Build stage
FROM rust:1.76 AS builder

WORKDIR /app
COPY . .

# Build the app in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install only what's needed to run
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/your-binary-name .

# Copy any AWS config/credentials if needed (optional)
# COPY ./aws/credentials /root/.aws/credentials
# COPY ./aws/config /root/.aws/config

# Expose the port your Actix app uses (e.g., 8080)
EXPOSE 8080

# Set the AWS region as an environment variable
ENV AWS_REGION=us-east-1

# Run the binary
CMD ["./your-binary-name"]

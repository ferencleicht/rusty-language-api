FROM rust:1.78.0-slim as builder

# Install system dependencies required for Diesel
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source and other necessary files
COPY src src
COPY migrations migrations

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder image
COPY --from=builder /app/target/release/rusty .

EXPOSE 8080

CMD ["./rusty"]

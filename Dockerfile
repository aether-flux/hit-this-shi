# ---- Build stage ----
FROM rust:latest as builder

WORKDIR /app

# Copy manifest files first
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual src
COPY src ./src

# Build real binary
RUN cargo build --release


# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install required system libs
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/hit-this-shi /app/hit-this-shi

# Expose the port your server listens on
EXPOSE 3000

# Run the binary
CMD ["./hit-this-shi"]


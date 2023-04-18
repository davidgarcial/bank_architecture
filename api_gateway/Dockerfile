# Use the official Rust image as the base image
FROM rust:1.68.2 as builder

# Install rustfmt
RUN rustup component add rustfmt

# Set the working directory
WORKDIR /api_gateway

# Copy the source code and configuration files into the container
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./proto ./proto
COPY ./.env ./.env
COPY ./build.rs ./build.rs

# Install the required dependencies
RUN cargo build --release

# Start a new build stage
FROM debian:buster-slim

# Install the necessary packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        tzdata \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /api_gateway/target/release/api_gateway /app/

# Expose the API gateway port
EXPOSE 5000

# Set the entrypoint to run the API gateway
ENTRYPOINT ["./api_gateway"]

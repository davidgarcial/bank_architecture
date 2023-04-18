# Use the official Rust base image
FROM rust:latest as builder

# Install rustfmt
RUN rustup component add rustfmt

# Create a new empty shell project
RUN USER=root cargo new --bin withdrawal_service
WORKDIR /withdrawal_service

# Copy your application's source code and dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./proto ./proto
COPY ./.env ./.env
COPY ./build.rs ./build.rs

# Install build dependencies and compile your application
RUN cargo build --release
RUN cargo install --path .

# Start a new stage to create the final image
FROM debian:buster-slim

# Install necessary libraries for gRPC
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary and .env file from the builder stage
COPY --from=builder /usr/local/cargo/bin/withdrawal_service /usr/local/bin/withdrawal_service
COPY --from=builder /withdrawal_service/.env /.env

# Set the working directory
WORKDIR /

# Expose the gRPC server port from the environment variable
ARG GRPC_SERVER_ADDRESS
ENV GRPC_SERVER_ADDRESS=${GRPC_SERVER_ADDRESS:-"0.0.0.0:50054"}
EXPOSE 50054

# Run the gRPC server
CMD ["withdrawal_service"]

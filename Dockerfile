# Stage 1: Build the application
FROM rust:1.88.0 AS builder

# Install libudev-dev
RUN apt-get update && apt-get install -y libudev-dev

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

ENV INDOCKER=true
# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/turret_mcp_server /usr/local/bin/turret_mcp_server

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/turret_mcp_server"]

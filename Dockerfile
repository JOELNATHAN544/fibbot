# Build stage
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the Rust project
RUN cargo build --release

# Final stage
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/release/fibbot .

# Expose the necessary port (if your program listens on a port)
EXPOSE 8080

# Command to run the binary
CMD ["./fibbot"]


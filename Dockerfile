# Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

COPY Cargo.toml Cargo.toml ./

# Copy the current directory contents into the container at /app
COPY . .

# Build the Rust project
RUN rustup update stable

RUN cargo build --release

# Copy the entrypoint script and make it executable
COPY script.sh /script.sh
#RUN chmod +x /script.sh

# Set the script to the entrypoint
ENTRYPOINT ["/app/target/release/fibbot"]


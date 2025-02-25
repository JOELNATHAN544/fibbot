FROM rust:latest

WORKDIR /usr/src/my_rust_project

COPY . .

RUN cargo build --release

CMD ["./target/release/my_rust_project"]

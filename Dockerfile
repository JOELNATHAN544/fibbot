FROM rust:latest

WORKDIR /usr/src/fibbot

COPY . .

RUN cargo build --release

CMD ["./target/release/fibbot"]

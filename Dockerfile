FROM rust:latest

WORKDIR /usr/src/fibbot

COPY . .

RUN cargo run


CMD ["./target/release/fibbot", "@0", "@1"]

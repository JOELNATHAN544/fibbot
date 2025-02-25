FROM rust:latest

WORKDIR /usr/src/fibbot

COPY . .

RUN cargo build --release

RUN ls -la /usr/src/fibbot


CMD ["./target/release/fibbot", "@0", "@1"]

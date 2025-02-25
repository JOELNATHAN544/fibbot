FROM rust:latest

WORKDIR /usr/src/fibbot

COPY . .

RUN cargo run 

CMD ["cargo", "run"]

FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo run

CMD ["cargo","run"]


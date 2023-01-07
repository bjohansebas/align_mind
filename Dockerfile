FROM rust:latest

RUN apt-get update && apt-get install -y libpq-dev

COPY . /usr/app
WORKDIR /usr/app

RUN cargo build --release

CMD ["./target/release/align_mind_server"]
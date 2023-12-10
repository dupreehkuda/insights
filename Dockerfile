FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update

COPY --from=builder /app/target/release/insights /usr/local/bin/insights

CMD ["insights"]
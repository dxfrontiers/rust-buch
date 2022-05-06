# 1: Build the exe
FROM rust:1.52.1 as builder
WORKDIR /usr/src

RUN USER=root cargo new rustbuch
WORKDIR /usr/src/rustbuch

COPY mailer/Cargo.toml ./.
COPY kafka-consumer /usr/src/kafka-consumer
RUN cargo build --release

# 1c: Build the exe using the actual source code
COPY mailer/src ./src
RUN rm -f target/release/deps/rustbuch*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/rustbuch/target/release/rustbuch_microservices_mailer ./mailer
COPY *.toml ./
USER 1000
CMD ["./mailer"]
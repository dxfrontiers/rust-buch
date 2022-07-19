FROM rust:1.62.1-buster as builder
WORKDIR /usr/src

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    apt-get install -y libldap-common

RUN USER=root cargo new rustbuch
WORKDIR /usr/src/rustbuch

COPY database-sink/Cargo.toml ./.
COPY kafka-consumer /usr/src/kafka-consumer
RUN cargo build --release

# 1c: Build the exe using the actual source code
COPY database-sink/src ./src
RUN rm -f target/release/deps/rustbuch*
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/rustbuch/target/release/rustbuch_microservices_database-sink ./rustbuch
COPY database-sink/diesel.toml ./
CMD ["./rustbuch"]
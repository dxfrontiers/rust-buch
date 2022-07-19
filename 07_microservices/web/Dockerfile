# 1: Build the exe
FROM rust:1.62.1-buster as builder
WORKDIR /usr/src

RUN USER=root cargo new rustbuch
WORKDIR /usr/src/rustbuch

COPY web/Cargo.toml ./.
RUN cargo build --release


# 1c: Build the exe using the actual source code
COPY web/src ./src
RUN rm -f target/release/deps/rustbuch*
RUN cargo build --release

FROM debian:buster
RUN apt update; \
    apt install -y --no-install-recommends \
    openssl
COPY --from=builder /usr/src/rustbuch/target/release/rustbuch_microservices_web ./web
COPY web/*.toml ./
CMD ["./web"]
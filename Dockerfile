FROM rust:alpine AS builder

WORKDIR /build
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:latest

WORKDIR /build
COPY --from=builder \
    /build/target/release/looker \
    /build

ENV DISCORD_TOKEN ""

ENTRYPOINT ./looker

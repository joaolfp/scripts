# syntax=docker/dockerfile:1

FROM rust:1.98.0-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
		git ca-certificates build-essential pkg-config \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
		git ca-certificates curl bash \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/scripts /usr/local/bin/scripts

ENTRYPOINT ["scripts"]

FROM rust:1.82-bullseye AS builder

WORKDIR /usr/src/quests-tracker

COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/opt/list/*

COPY --from=builder /usr/local/cargo/bin/quests-tracker /usr/local/bin/quests-tracker

CMD ["quests-tracker"]
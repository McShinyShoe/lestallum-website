# syntax=docker/dockerfile:1.7

FROM rust:1-bookworm AS builder

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos

WORKDIR /build

COPY package.json package-lock.json ./
RUN npm ci

COPY Cargo.toml Cargo.lock ./
COPY app/Cargo.toml app/Cargo.toml
COPY frontend/Cargo.toml frontend/Cargo.toml
COPY server/Cargo.toml server/Cargo.toml

COPY app/src app/src
COPY frontend/src frontend/src
COPY server/src server/src
COPY style style
COPY public public
COPY tailwind.config.js ./

RUN cargo leptos build --release


FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --shell /usr/sbin/nologin app

WORKDIR /app

COPY --from=builder /build/target/release/server /app/server
COPY --from=builder /build/target/site /app/site

ENV LEPTOS_OUTPUT_NAME="lestallum-website" \
    LEPTOS_SITE_ROOT="/app/site" \
    LEPTOS_SITE_PKG_DIR="pkg" \
    LEPTOS_SITE_ADDR="0.0.0.0:3000" \
    LEPTOS_ENV="PROD"

USER app
EXPOSE 3000

CMD ["/app/server"]

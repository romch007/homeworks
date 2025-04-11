FROM node:lts-alpine AS frontend

WORKDIR /app

RUN npm install -g pnpm

COPY frontend/package.json frontend/pnpm-lock.yaml ./

RUN pnpm install

COPY frontend ./

RUN ls

RUN pnpm run build

FROM rust:1-alpine AS backend

ENV RUSTFLAGS='-C target-feature=-crt-static'

RUN apk update --no-cache && apk add --no-cache musl-dev curl

RUN cargo install cargo-build-deps

WORKDIR /app

RUN cargo new --bin homeworks
WORKDIR /app/homeworks

COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release

COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

ENV TINI_VERSION=v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /tini
RUN chmod +x /tini

FROM alpine

RUN addgroup -S appgroup && adduser -S appuser -G appgroup

RUN apk update --no-cache && apk add --no-cache libgcc

ENV ADDR=0.0.0.0
ENV PORT=8080

EXPOSE 8080

WORKDIR /app

COPY --from=frontend /app/dist ./dist/
COPY --from=backend /app/homeworks/target/release/homeworks ./
COPY --from=backend /tini /tini

USER appuser

ENTRYPOINT ["/tini" , "--"]

CMD ["/app/homeworks"]


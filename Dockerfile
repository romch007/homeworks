FROM node:lts-alpine AS frontend

WORKDIR /app

RUN npm install -g pnpm

COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install

COPY frontend ./
RUN pnpm run build

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app/homeworks

FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY src src/
COPY migrations migrations/

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend

COPY --from=planner /app/homeworks/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY src src/
COPY migrations migrations/

RUN cargo build --release && \
    strip target/release/homeworks

ENV TINI_VERSION=v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /tini
RUN chmod +x /tini

FROM gcr.io/distroless/cc-debian12:nonroot

ENV ADDR=0.0.0.0
ENV PORT=8080

EXPOSE 8080

WORKDIR /app

COPY --from=frontend /app/dist ./dist/
COPY --from=backend /app/homeworks/target/release/homeworks ./
COPY --from=backend /tini /tini

ENTRYPOINT ["/tini", "--"]

CMD ["/app/homeworks"]

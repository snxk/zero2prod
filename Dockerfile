FROM rust:1.57.0 as build

WORKDIR /zero2prod
COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=build /zero2prod/target/release/zero2prod zero2prod
COPY config config

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
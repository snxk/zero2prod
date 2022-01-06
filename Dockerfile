FROM rust:1.57.0 as build

RUN USER=root cargo new --bin zero2prod
WORKDIR /zero2prod

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY sqlx-data.json sqlx-data.json

RUN rm ./target/release/deps/zero2prod*
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=build /zero2prod/target/release/zero2prod /usr/local/bin/zero2prod

CMD ["/usr/local/bin/zero2prod"]
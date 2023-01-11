# syntax=docker/dockerfile:1

##
## Build
##
FROM rust:latest as build

RUN cargo new taskot
WORKDIR /taskot/

# Cache dependencies
COPY Cargo.lock ./
COPY Cargo.toml ./
RUN cargo build --release
RUN rm src/*

# Actual build
COPY src/ ./src/
RUN rm ./target/release/deps/taskot*
RUN cargo build --release

##
## Deploy
##

CMD ["target/release/taskot"]

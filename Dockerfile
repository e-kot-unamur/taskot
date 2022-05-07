# syntax=docker/dockerfile:1

##
## Build
##
FROM rust:1.60 as build

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
FROM gcr.io/distroless/cc

COPY --from=build /taskot/target/release/taskot ./

ENTRYPOINT ["./taskot"]

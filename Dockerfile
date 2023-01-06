# syntax=docker/dockerfile:1

##
## Build
##
FROM rust:1.60 as build

RUN mkdir taskot
WORKDIR /taskot/

# Cache dependencies
COPY . .
RUN cargo build --release

# Actual build
RUN rm ./target/release/deps/taskot*
RUN cargo build --release

##
## Deploy
##

CMD ["cargo","run","--release"]

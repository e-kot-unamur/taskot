# syntax=docker/dockerfile:1

##
## Build
##
FROM rust:1.60 as build

RUN mkdir taskot
WORKDIR /taskot/

# BUILD
COPY . .
RUN cargo build --release

##
## Deploy
##

CMD ["cargo","run","--release"]

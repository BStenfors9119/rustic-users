FROM rust:latest

RUN USER=root cargo new --bin rustic-users
WORKDIR /rustic-users

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./.env.stage ./.env

RUN pwd
RUN ls -al src
RUN ls target/release
RUN rm target/release/deps/rustic_users*
RUN cargo build --bins --release



#FROM ubuntu:latest
#
#ENV DEBIAN_FRONTEND=noninteractive
#
#RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
#
#COPY --from=0 /build-out/rustic_auth /
EXPOSE 50051

CMD target/release/rustic_users_server

FROM rust:latest as builder
WORKDIR /usr/src/echo-server
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src ./src
RUN cargo build --release

EXPOSE 3000
CMD ["/usr/src/echo-server/target/release/echo-server"]

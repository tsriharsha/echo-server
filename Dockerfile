FROM rust:alpine3.17 as builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/echo-server
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src ./src
RUN cargo build --release

FROM alpine:3.17
COPY --from=builder /usr/src/echo-server/target/release/echo-server /usr/src/echo-server/target/release/echo-server
CMD ["/usr/src/echo-server/target/release/echo-server"]
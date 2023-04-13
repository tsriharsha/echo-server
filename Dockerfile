FROM rust:1.68-slim as builder
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/echo-server
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src ./src
RUN cargo build --release

FROM rust:1.68-slim
COPY --from=builder /usr/src/echo-server/target/release/echo-server /usr/src/echo-server/target/release/echo-server
RUN chmod +x /usr/src/echo-server/target/release/echo-server
EXPOSE 3000
CMD ["/usr/src/echo-server/target/release/echo-server"]
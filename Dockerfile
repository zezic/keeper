FROM rust:1.52.1-slim-buster as builder

RUN apt update && apt -y install build-essential

WORKDIR app
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD ./src ./src
RUN cargo build --release

FROM rust:1.52.1-slim-buster as runtime

EXPOSE 3030

COPY --from=builder /app/target/release/keeper /usr/local/bin

ENTRYPOINT ["./usr/local/bin/keeper"]
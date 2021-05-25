FROM ekidd/rust-musl-builder:1.51.0 as builder

RUN mkdir keeper
WORKDIR ./keeper
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD ./src ./src
RUN cargo build --release

FROM alpine:3.13.5

EXPOSE 3030

COPY --from=builder /home/rust/src/keeper/target/x86_64-unknown-linux-musl/release/keeper /usr/src/app/keeper

WORKDIR /usr/src/app

CMD ["./keeper"]
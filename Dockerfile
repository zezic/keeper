FROM ekidd/rust-musl-builder:1.51.0 as builder

# Cached layer with Cargo index and deps
RUN cargo new --bin keeper
WORKDIR ./keeper
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rm -rf target

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD ./src ./src
RUN cargo build --release

FROM alpine:3.13.5

EXPOSE 3030

# RUN apk update \
#     && apk add --no-cache ca-certificates tzdata \
#     && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/keeper/target/x86_64-unknown-linux-musl/release/keeper /usr/src/app/keeper

WORKDIR /usr/src/app

CMD ["./keeper"]
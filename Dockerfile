FROM rust:slim-bookworm AS builder
ARG TARGET=x86_64-unknown-linux-musl

RUN rustup target add $TARGET

WORKDIR /build
COPY Cargo.toml Cargo.lock .
RUN cargo fetch --locked --target $TARGET
COPY src src
RUN cargo install --path . --target $TARGET

FROM scratch
COPY --from=builder /usr/local/cargo/bin/halo-world /app
CMD ["/app"]

FROM rust:alpine3.14 as build
RUN apk add musl-dev

COPY ./src/ ./src/
COPY ./Cargo.toml ./Cargo.toml
RUN cargo install --path . --target=x86_64-unknown-linux-musl
# RUN cp target/release/palettify /build-out/
# CMD ["/bin/sh"]

FROM alpine:3.14
# # RUN apt-get update && apt-get install -y zstd libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/* /usr/local/bin

EXPOSE 8080

CMD ["palettify"]
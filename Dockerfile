FROM ghcr.io/penumbra-x/rust-musl-cross:x86_64-unknown-linux-musl AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:3.16

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/duckai /bin/duckai

CMD ["/bin/duckai"]

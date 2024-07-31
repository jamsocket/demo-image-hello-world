FROM rust:1.80-slim as builder
WORKDIR /work
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /work/target/release/demo-image-hello-world /usr/local/bin/demo-image-hello-world
CMD ["demo-image-hello-world"]

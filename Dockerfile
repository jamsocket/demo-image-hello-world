FROM rust:1.68-slim as builder
WORKDIR /work
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/base-debian11
COPY --from=builder /work/target/release/demo-image-hello-world /usr/local/bin/demo-image-hello-world
CMD ["demo-image-hello-world"]

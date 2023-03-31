FROM rust:1.68-alpine as builder
WORKDIR /work
COPY . .
RUN cargo install --path .

FROM alpine:3.16
COPY --from=builder /work/demo-image-hello-world /usr/local/bin/demo-image-hello-world
CMD ["demo-image-hello-world"]

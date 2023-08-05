FROM rust:latest AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/calculator
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/local/cargo/bin/rust_calculator /usr/local/bin/rust_calculator

EXPOSE 8080

CMD ["rust_calculator"]

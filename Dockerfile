# Compile binary
FROM rust:1.51 as builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential git clang llvm-dev libclang-dev libssl-dev pkg-config libpq-dev brotli
COPY Cargo.toml Cargo.lock ./
COPY src ./src
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"
RUN cargo install --path .

# Copy to image
FROM debian:buster-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/local/cargo/bin/tracing-tui /bin
RUN apt-get update && apt-get install -y libssl-dev pkg-config libpq-dev brotli
CMD [ "/bin/tracing-tui" ]

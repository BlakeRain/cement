FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update -y
RUN apt install -y musl-tools musl-dev

WORKDIR /usr/src
RUN USER=root mkdir cement
WORKDIR /usr/src/cement
COPY Cargo.toml ./
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY build.rs package.json tailwind.config.js ./
COPY migrations src style templates ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/cement .
USER 1000
ENTRYPOINT ["./cement"]

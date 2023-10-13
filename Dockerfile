FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update -y
RUN apt install -y musl-tools musl-dev ca-certificates curl gnupg

RUN mkdir -p /etc/apt/keyrings
RUN curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
RUN apt-get update -y
RUN apt-get install -y nodejs

WORKDIR /usr/src
RUN USER=root cargo new cement
WORKDIR /usr/src/cement
COPY Cargo.toml ./
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY package.json tailwind.config.js ./
RUN npm install

RUN rm src/main.rs
COPY build.rs ./
COPY migrations ./migrations
COPY src ./src
COPY templates ./templates
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM debian:buster-slim
RUN apt-get update -y
RUN apt-get install -y ca-certificates tzdata
RUN rm -rf /var/lib/apt/lists/*

EXPOSE 8000

COPY --from=builder /usr/src/cement/target/release/cement .
CMD ["./cement"]

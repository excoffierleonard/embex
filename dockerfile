# Step 1: Build the application with musl target
FROM rust:alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

WORKDIR /embex

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/main.rs

COPY src src/

RUN touch src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM alpine

WORKDIR /embex

COPY --from=builder /embex/target/x86_64-unknown-linux-musl/release/embex .

CMD ["./embex"]
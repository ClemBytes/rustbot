# Step 1: build app
FROM rust:1.89 AS builder
WORKDIR /app
# musl for static build
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates templates
RUN cargo build --release --target x86_64-unknown-linux-musl

# Step 2: minimal final image
FROM scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rustbot /app/rustbot
COPY static static
CMD ["/app/rustbot"]

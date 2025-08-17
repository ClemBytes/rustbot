# Step 1: build app
FROM rust:1.83 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates templates
RUN cargo build --release

# Step 2: final image (not minimal, because requires static build)
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rustbot /app/rustbot
COPY static static
CMD ["/app/rustbot"]

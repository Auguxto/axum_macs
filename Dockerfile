FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/axum_macs /app/axum_macs
CMD [ "/app/axum_macs" ]
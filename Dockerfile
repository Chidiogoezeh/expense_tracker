# Build the Rust application
FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY src ./src
COPY migrations ./migrations

ENV SQLX_OFFLINE=true

RUN cargo build --release

# Stage 2: Run the compiled application
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home appuser

WORKDIR /app

COPY --from=builder /app/target/release/expense_tracker /app/expense_tracker

USER 10001:10001

EXPOSE 3000

CMD ["/app/expense_tracker"]

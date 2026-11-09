FROM rust:1.90-slim AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Cache dependencies as a separate layer
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN useradd --system --uid 10001 app
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
USER app
ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
CMD ["backend"]

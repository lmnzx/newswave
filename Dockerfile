FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin newswave

FROM gcr.io/distroless/cc-debian11 as runtime
WORKDIR /app
COPY --from=builder /app/target/release/newswave newswave
COPY config config
ENV MODE production
ENTRYPOINT ["./newswave"]
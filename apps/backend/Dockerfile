### build ###

FROM rust:1.92 AS builder

RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app

# optimize deps cache
COPY Cargo.toml Cargo.lock .
COPY apps/backend/Cargo.toml apps/backend/Cargo.toml

RUN mkdir apps/backend/src
RUN echo 'fn main() { panic!("NO!"); }' > apps/backend/src/main.rs

RUN cargo build --release -p backend

RUN rm -rf apps/backend/src
# ----------------------

COPY . .

# needed to update the timestamp, without it it won't detect changes
RUN touch apps/backend/src/main.rs

RUN cargo build --release -p backend

### run ###

FROM debian:bookworm-slim AS runner

WORKDIR /app

COPY --from=builder /app/target/release/backend ./

EXPOSE 3000

CMD ["./backend"]
### build ###

FROM rust:1.92 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock .
COPY apps/backend/Cargo.toml apps/backend/Cargo.toml

RUN mkdir apps/backend/src
RUN echo 'fn main() { panic!("NO!"); }' > apps/backend/src/main.rs

RUN cargo build --release -p backend -v

RUN rm -rf apps/backend/src

COPY . .

# needed to update the timestamp, without it it won't detect changes
RUN touch apps/backend/src/main.rs

RUN cargo build --release -p backend -v

### run ###

FROM debian:bookworm-slim AS runner

WORKDIR /app

COPY --from=builder /app/target/release/backend ./

EXPOSE 3000

CMD ["./backend"]
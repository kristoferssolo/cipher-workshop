FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rustlang/rust:nightly-bookworm AS cacher

RUN curl -LO https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos cargo-chef -y
RUN apt-get update -y && apt-get install -y --no-install-recommends clang
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly-bookworm AS builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y
RUN apt-get update -y && apt-get install -y --no-install-recommends clang
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY --from=cacher /app/target target
COPY . .
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/web /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

EXPOSE 8080

CMD ["/app/web"]

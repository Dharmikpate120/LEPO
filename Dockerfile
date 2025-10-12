FROM rust AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY Cargo.lock Cargo.toml ./
RUN cargo chef prepare --recipe-path recipe.json


FROM rust AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR /app
COPY --from=cacher /app/target target
COPY . .
RUN cargo build --release

ENTRYPOINT ["/app/target/release/lepo"]
# CMD ["cargo","run"]
# RUN --mount=type=cache,target=/root/cargo/cache \
#     --mount=type=cache,target=/lepo/target \
#     mkdir -p src && echo "fn main() {}" > src/main.rs \
#     cargo build --release

# COPY src ./src
# COPY migrations ./migrations
# COPY .env ./.env
# 
# RUN --mount=type=cache,target=/lepo/target \
    # cargo build --release
# 
# EXPOSE 8000

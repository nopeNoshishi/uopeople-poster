# Build
FROM rust:1.67 as builder

# First build for cache
RUN cargo new --bin api
WORKDIR /api
COPY /api/Cargo.toml Cargo.toml
RUN cargo build --release

# Second build for application
ADD ./api/src ./src
RUN rm ./target/release/deps/api*
RUN cargo build --release

# Deploy
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq-dev

RUN mkdir -p usr/src/app
COPY --from=builder /api/target/release/api usr/src/app/api
WORKDIR /usr/src/app
CMD [ "./api" ]

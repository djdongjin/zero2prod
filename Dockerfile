FROM rust:1.72.0

WORKDIR /app

RUN apt update && apt install -y lld clang

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT [ "./target/release/zero2prod" ]
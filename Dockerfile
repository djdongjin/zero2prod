FROM rust:1.77.0

WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./target/release/zero2prod" ]
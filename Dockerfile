FROM rust:1.55.0

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./target/release/newsletter"]

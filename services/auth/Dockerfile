FROM rust:1.64-alpine

WORKDIR /usr/src/auth
COPY . .

RUN apk add --no-cache musl-dev

RUN cargo install --path .

CMD ["auth"]

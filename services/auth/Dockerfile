FROM rust:1.64

WORKDIR /usr/src/auth
COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["auth"]

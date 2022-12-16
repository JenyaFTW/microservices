FROM rust:1.64

WORKDIR /usr/src/shipment
COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["auth"]

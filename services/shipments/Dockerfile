FROM rust:1.65

WORKDIR /usr/src/shipment
COPY . .

RUN cargo install --path .

EXPOSE 3005

CMD ["shipment"]

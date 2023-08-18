FROM rust:1.69.0-alpine3.17

WORKDIR /gumma-auth
COPY . .

RUN cargo install --path .

ENV RUST_LOG=plupp

CMD ["gumma-auth"]
FROM rust:1.64

WORKDIR /usr/src/gumma-auth
COPY . .

RUN cargo install --path .

ENV RUST_LOG=plupp

CMD ["gumma-auth"]
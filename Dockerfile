FROM rust:1 AS builder
WORKDIR /usr/src/etoast
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new etoast
WORKDIR /usr/src/etoast
RUN mkdir ./src && echo "fn main() {}" > ./src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --target x86_64-unknown-linux-musl
RUN rm ./src/*.rs

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/etoast .
COPY static ./static
COPY templates ./templates
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
USER 1000
CMD [ "./etoast" ]

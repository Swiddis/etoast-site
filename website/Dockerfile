FROM rust:1.68-alpine
WORKDIR /usr/src/etoast
COPY . .
RUN cargo install --path .
CMD [ "etoast" ]

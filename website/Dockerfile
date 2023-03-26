FROM rust:1.68
WORKDIR /usr/src/etoast
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
COPY . .
RUN cargo install --path .
CMD [ "etoast" ]

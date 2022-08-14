FROM golang:1.17.0 AS build
WORKDIR /app
COPY go.* ./
COPY main.go .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o app .

FROM alpine:latest AS prod
WORKDIR /app
COPY  ./static ./static
COPY --from=build /app/app ./app
EXPOSE 8080
CMD ./app

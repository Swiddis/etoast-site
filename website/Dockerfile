FROM golang:1.17.0 AS build
WORKDIR /app
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o app .

FROM alpine:latest AS prod
WORKDIR /app
EXPOSE 8080:8080
COPY --from=build /app/static ./static
COPY --from=build /app/app ./app
CMD ./app


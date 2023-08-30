FROM node:lts AS builder

WORKDIR /app
COPY . .

RUN yarn install \
    --prefer-offline \
    --frozen-lockfile \
    --non-interactive \
    --production=false
RUN yarn build

FROM nginx

COPY --from=builder /app/dist /usr/share/nginx/html

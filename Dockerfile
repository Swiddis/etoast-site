FROM node:lts AS builder

WORKDIR /app

RUN yarn install \
    --prefer-offline \
    --frozen-lockfile \
    --non-interactive \
    --production=false
COPY . .
RUN yarn build

FROM nginx

COPY ./_nginx /etc/nginx
COPY --from=builder /app/dist /www/data

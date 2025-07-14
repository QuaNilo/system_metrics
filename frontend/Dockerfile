FROM node:24.4.0-alpine3.22 AS builder

WORKDIR /app
COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

FROM nginx:stable-alpine3.17-slim

ARG USER_ID=1001
ARG GROUP_ID=1001

RUN addgroup -g $GROUP_ID nginxgroup && adduser -u $USER_ID -G nginxgroup -S nginxuser

RUN mkdir -p /usr/share/nginx/html /var/cache/nginx/client_temp /var/run/nginx \
    && chown -R nginxuser:nginxgroup /usr/share/nginx/html /var/cache/nginx /var/run/nginx

COPY nginx_global.conf /etc/nginx/nginx.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html

RUN apk update && \
    apk upgrade && \
    apk add --no-cache bash

USER nginxuser

EXPOSE 50001

CMD ["nginx", "-g", "daemon off;"]


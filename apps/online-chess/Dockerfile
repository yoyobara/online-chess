### build ###

FROM node:22 AS builder

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm ci

COPY . . 

RUN npx nx run online-chess:build --prod

### run ###

FROM nginx:stable

WORKDIR /app

COPY --from=builder /app/apps/online-chess/dist /usr/share/nginx/html

COPY apps/online-chess/nginx_server.conf /etc/nginx/conf.d/chess_online.conf
RUN rm /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
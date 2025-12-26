### build ###

FROM node:22 AS builder

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm ci

COPY . . 

RUN npx nx run online-chess:build --prod

### run ###

FROM nginx:stable

COPY --from=builder /app/apps/online-chess/dist /usr/share/nginx/html

RUN rm /etc/nginx/conf.d/default.conf
COPY apps/online-chess/nginx_server.conf.template /etc/nginx/templates/chess_online.conf.template

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
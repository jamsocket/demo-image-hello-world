FROM node:lts-alpine3.16

EXPOSE 8080

WORKDIR /usr/src/app

COPY server.js /usr/src/app

CMD ["node", "server.js"]

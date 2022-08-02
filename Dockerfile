FROM node:18-alpine3.15

EXPOSE 8080

WORKDIR /usr/src/app

COPY server.js /usr/src/app

CMD ["node", "server.js"]

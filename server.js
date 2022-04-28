const http = require('http');

const HOSTNAME = '0.0.0.0';
const port = process.env.PORT || 8000;

const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'application/json');

  let response = {
    message: "Hello World",
    port: process.env.PORT || null,
    spawner_service: process.env.SPAWNER_SERVICE || null,
    spawner_url: process.env.SPAWNER_URL || null,
    spawner_name: process.env.SPAWNER_NAME || null,
  };  

  res.end(JSON.stringify(response));
});

server.listen(port, HOSTNAME, () => {
  console.log(`Server running at http://${HOSTNAME}:${port}/`);
});


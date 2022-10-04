const http = require('http');

const HOSTNAME = '0.0.0.0';
const PORT = parseInt(process.env.PORT || 8000);

const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'application/json');

  let response = {
    message: "Hello World",
    port: PORT,
    jamsocket_service: process.env.JAMSOCKET_SERVICE || null,
    jamsocket_url: process.env.JAMSOCKET_URL || null,
    jamsocket_name: process.env.JAMSOCKET_NAME || null,
    arbitrary_env_var: process.env.ARBITRARY_ENV_VAR,
  };  

  res.end(JSON.stringify(response));
});

server.listen(PORT, HOSTNAME, () => {
  console.log(`Server running at http://${HOSTNAME}:${PORT}/`);
});


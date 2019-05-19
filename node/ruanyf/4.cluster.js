const cluster = require('cluster');
const os = require('os');
const http = require('http');

{
  if (cluster.isMaster) {
    for (let i = 0, n = os.cpus().length; i < n; i += 1) {
      cluster.fork();
    }
  } else {
    http.createServer(function (req, res) {
      res.writeHead(200);
      res.end("hello world\n");
    }).listen(8000);
  }
}
const cluster = require('cluster');
const os = require('os');
const http = require('http');

{
  /*
  if (cluster.isMaster) {
    for (let i = 0, n = os.cpus().length; i < n; i += 1) {
      cluster.fork();
    }
  } else {
    http.createServer(function (req, res) {
      res.writeHead(200);
      res.end("hello world\n");
    }).listen(8000, () => {
      console.log('cluster.isMaster is ', cluster.isMaster)
    });
  }
  */
}

{
  console.log('begin run');
  if(cluster.isMaster) {
    let numWorkers = os.cpus().length;
    console.log('Master cluster setting up ' + numWorkers + ' workers...');

    for(let i = 0; i < numWorkers; i++) {
      cluster.fork();
    }

    cluster.on('online', function(worker) {
      console.log('Worker ' + worker.process.pid + ' is online');
    });

    cluster.on('exit', function(worker, code, signal) {
      console.log('Worker ' + worker.process.pid + ' died with code: ' + code + ', and signal: ' + signal);
      console.log('Starting a new worker');
      cluster.fork();
    });
  }
}
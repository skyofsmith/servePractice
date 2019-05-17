const childProcess = require('child_process');
const exec = childProcess.exec;

// 1.exec
{
  let ls = exec('ls -l', (err, stdout, stderr) => {
    if (err) {
      console.log(JSON.stringify(err.stack));
      console.log('Error code: ' + JSON.stringify(err.code));
      console.log('STDERR: ' + JSON.stringify(stderr));
    }
    console.log('Child Process STDOUT: ' + stdout);
  });
  console.log(ls);

  let child = exec('ls -l');

  child.stdout.on('data', data => {
    console.log('stdout: ' + data);
  });
  child.stderr.on('data', data => {
    console.log('stderr: ' + data);
  });
  child.on('close', data => {
    console.log('closing code: ' + data);
  });

  let path = '; user input unsafe';
  exec('ls -l ' + path, (err, data) => {
    console.log(data)
  })
}


const childProcess = require('child_process');
const path = require('path');
const exec = childProcess.exec;
const execSync = childProcess.execSync;
const execFile = childProcess.execFile;

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

// 2.execSync
{
  let SEPARATOR = process.platform === 'win32' ? ';' : ':';
  let env = Object.assign({}, process.env);

  env.PATH = path.resolve('./node_modules/.bin') + SEPARATOR + env.PATH;

  function myExecSync(cmd) {
    console.log('process.cwd() is ', process.cwd());
    console.log('__dirname is ', __dirname);
    console.log('process.cwd() === __dirname is ', process.cwd() === __dirname);
    let output = execSync(cmd, {
      cwd: process.cwd(),
      env: env
    });

    console.log(output);
  }

  myExecSync('eslint --version');
}

// 3.execFile
{
  let path = '.';
  execFile('/bin/ls', ['-l', path], (err, result) => {
    if (err) {
      console.log(err)
    }
    console.log(result)
  })
}

// 4.spawn
{

}

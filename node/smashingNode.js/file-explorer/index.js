/**
 * Module dependencies.
 */

var fs = require('fs');
var stdout = process.stdout;
var stdin = process.stdin;
fs.readdir(process.cwd(), function(err, files) {
  console.log('');

  if (!files.length) {
    return console.log('    \033[31m No files to show!\033[39m\n');
  }

  console.log('    Select which file or directory you want to see\n');

  function file(i) {
    var filename = files[i];

    fs.stat(__dirname + '/' + filename, function(err, stat) {
      if (stat.isDirectory()) {
        console.log('    ' + i + '    \033[36m' + filename + '/\033[39m\n');
      } else {
        console.log('    ' + i + '    \033[90m' + filename + '\033[39m\n');
      }

      i++;
      if (i === files.length) {
        read();
      } else {
        file(i);
      }
    })
  }

  file(0);
})

function read() {
  console.log('');
  stdout.write('    \033[33mEnter your choice: \033[39m\n');
  stdin.resume();
  stdin.setEncoding('utf-8');
}

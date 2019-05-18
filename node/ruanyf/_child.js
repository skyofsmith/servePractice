console.log('child.js run begin');
process.on('message', function(m) {
  console.log('CHILD got message:', m);
});
process.send({ foo: 'bar' });
console.log('child.js run end');
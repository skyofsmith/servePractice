var querystring = require('querystring');
var s1 = querystring.stringify({name: 'sam', age: 27});
console.log(s1);    //name=sam&age=27
var s2 = querystring.stringify({name: 'sam', age: 27}, ',');
console.log(s2);    //name=sam,age=27
var s3 = querystring.stringify({name: 'sam', age: 27}, ',', ':');
console.log(s3);    //name:sam,age:27

var p1 = querystring.parse(s1);
console.log(p1);
var p2 = querystring.parse(s2, ',');
console.log(p2);
var p3 = querystring.parse(s3, ',', ':');
console.log(p3);

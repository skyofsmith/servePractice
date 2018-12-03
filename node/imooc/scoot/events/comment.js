var http = require('http');
var querystring = require('querystring');

var postData = querystring.stringify({
    'content': 'hahaha',
    'cid': 123
});

var options = {
    hostname: 'www.imooc.com',
    port: 80,
    path: '/course/document',
    method: 'POST',
    headers: {
        'Accept': 'application/json, text/javascript'
    }
};

var req = http.request(options, function(res) {
    console.log('Status: ' + res.statusCode);
    console.log('headers: ' + JSON.stringify(res.headers));

    res.on('data', function(chunk) {
        console.log(Buffer.isBuffer(chunk));
        console.log(typeof chunk);
    });

    res.on('end', function(chunk) {
        console.log('comment finished!');
    });

    req.write(postData);
    req.end();
});

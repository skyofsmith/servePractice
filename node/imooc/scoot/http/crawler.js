var http = require('http');
var cheerio = require('cheerio');
var url = 'http://tieba.baidu.com/p/3182877071';

function filterChapters(html) {
    var $ = cheerio.load(html);
    var chapters = $('.more');
}
http
    .get(url, function(res) {
        var html = '';

        res.on('data', function(data) {
            html += data;
        });
        res.on('end', function(data) {
            console.log(html);
            filterChapters(html);
        });
    })
    .on('error', function() {
        console.log('获取数据出错！');
    })
    ;
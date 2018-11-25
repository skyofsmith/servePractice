var url = require('url');
var url1 = 'https://www.imooc.com:8000/video/6710?a=1&b=2#/c1';
var res = url.parse(url1);
console.log(res);
/*
Url {
    protocol: 'https:',             //协议
    slashes: true,                  //是否有双斜线
    auth: null,                     //
    host: 'www.imooc.com:8000',     //域名(带端口号)
    port: '8000',                   //端口号
    hostname: 'www.imooc.com',      //主机名
    hash: '#/c1',                   //锚点
    search: '?a=1&b=2',             //查询字符串参数(带?)
    query: 'a=1&b=2',               //查询字符串参数
    pathname: '/video/6710',        //资源路径名
    path: '/video/6710?a=1&b=2',    //路径(带search)
    href: 'https://www.imooc.com:8000/video/6710?a=1&b=2#/c1    //完整路径
}
*/

var path = url.format(res);
console.log(path);
//parse 与 format互逆

var nPath = url.resolve('http://about.com/', '/course/list');
console.log(nPath);

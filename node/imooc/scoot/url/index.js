var url = require('url');
var url1 = 'https://www.imooc.com:8000/video/6710?a=1&b=2#/c1';
var res = url.parse(url1);
console.log(res);
/*
Url {
    protocol: 'https:',             //协议
    slashes: true,                  //是否有双斜线
    auth: null,                     //
    host: 'www.imooc.com',          //域名(带端口号)
    port: null,                     //端口号
    hostname: 'www.imooc.com',      //主机名
    hash: null,                     //锚点
    search: null,                   //查询字符串参数(带?)
    query: null,                    //查询字符串参数
    pathname: '/video/6710',        //资源路径名
    path: '/video/6710',            //路径(带search)
    href: 'https://www.imooc.com/video/6710'    //完整路径
}
*/

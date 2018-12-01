/*
    输入url,回车
    1.浏览搜索自身DNS缓存    (chrome://net-internals/#dns)
    2.搜索操作系统自身DNS缓存
    3.读取本地HOST文件
    4.浏览器发起一个DNS的一个系统调用
        1)宽带运营商服务器查看本身缓存
        2）运营商服务器发起一个迭代DNS解析请求
        运营商服务器吧结果返回操作系统内核同时缓存起来
        操作系统内核吧结果返回浏览器
        最终浏览器拿到了网址对应的ip地址
    5.浏览器获得域名对应的IP地址后,发起HTTP"三次握手"
    6.TCP/IP链接建立起来后，浏览器就可以想服务器发送HTTP请求了 比如说 使用HTTP的GET方法请求一个根域里的一个域名， 协议采用HTTP 1.0的一个协议
    7.服务器端接收了这个请求， 根据路径参数， 经过后端的一些处理之后， 把处理后的结果返回给浏览器， 如果是慕课网的页面就会吧完整的HTML页面代码返回给浏览器
    8.浏览器拿到了慕课网的完整的HTML页面代码， 在解析和渲染这个页面的时候， 里面的JS、CSS、图片静态资源， 他们同样也是一个个HTTP请求都需要经过上面的主要的七个步骤
    9.浏览器根据拿到的资源对页面进行渲染， 最终把一个完整的页面呈现给了用户
*/
/*
    request
    请求方式：
    GET
    POST
    PUT
    DELETE
    HEAD
    TRACE
    OPTIONS

    请求组成：
请求方式 url http版本
key: value
...

请求body


    response
    响应组成：
    http版本 响应状态码 响应成功或失败
    key: value
    ...

    响应body

    响应码
    1xx 
    2xx 成功
    3xx 重定向
    4xx 客户端失败
    5xx 服务端失败
*/
/**/
const http = require('http');
const fs = require('fs');
const _ = require('lodash');

function saveData (str, m) {
  // let dataObj = data[m];
  fs.writeFileSync('./baike.history.today/' + m + '.json', str)
}
function queryData(month) {
  let monthStr = month < 10 ? '0' + month : '' + month;
  let url = 'http://baike.baidu.com/cms/home/eventsOnHistory/' + monthStr + '.json';
  http.get(url, (res) => {
    const { statusCode } = res;
    // const contentType = res.headers['content-type'];

    let error;
    if (statusCode !== 200) {
      error = new Error('请求失败\n' +
        `状态码: ${statusCode}`);
    // } else if (!/^application\/json/.test(contentType)) {
    //   error = new Error('无效的 content-type.\n' +
    //     `期望的是 application/json 但接收到的是 ${contentType}`);
    }
    if (error) {
      console.error(error.message);
      // 消费响应数据来释放内存。
      res.resume();
      return;
    }

    res.setEncoding('utf8');
    let rawData = '';
    res.on('data', (chunk) => { rawData += chunk; });
    res.on('end', () => {
      saveData(rawData, monthStr);
    });
  }).on('error', (e) => {
    console.error(`出现错误: ${e.message}`);
  });
}
_.chain(1).range(13).each(queryData).value();

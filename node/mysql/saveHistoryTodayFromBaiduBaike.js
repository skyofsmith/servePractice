const http = require('http');
const fs = require('fs');
const _ = require('lodash');
const mysql = require('mysql');

const connection = mysql.createConnection({
  host: 'localhost',
  port: 3306,
  user: 'root',
  password: 'root',
  database: 'test'
});
connection.connect();
function html2text(html) {
  let div = document.createElement('div');
  div.innerHTML = html;
  return div.innerText;
}
function saveData2mysql_lite (obj, m, d) {
  let ms = m * 1;
  let ds = d * 1;
  let objs = JSON.stringify(obj);
  let query = connection.query(
    'insert into history_today_baike_lite(month, day, content) values(?, ?, ?)',
    [ms, ds, objs],
    (err, result) => {
    if (err) {
      console.log(err);
    }
  });
  console.log('sql is: ', query.sql);
}
function saveData2mysql (arr, m, d) {
  // connection.query('insert ')
  let ms = m * 1;
  let ds = d * 1;
  _.each(arr, obj => {
    let { cover, desc, festival, link, recommend, title, year, type } = obj;
    let ys = year * 1;
    let query = connection.query(
      'insert into history_today_baike(month, day, year, title, festival, link, type, description, cover, recommend) values(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)',
      [ms, ds, ys, title, festival, link, type, desc, cover, recommend],
      (err, result) => {
        if (err) {
          console.log(err);
        }
      });
    console.log('sql is: ', query.sql);
  });
  saveData2mysql_lite(arr, m, d);
}
function saveData(rawData, monthStr) {
  let result = null;
  try {
    result = JSON.parse(rawData);
  } catch (e) {
    console.log(e);
  }
  let arr = result[monthStr];
  _.each(arr, (obj, key) => {
    let [mon, day] = key.match(/\d{2}/g);
    saveData2mysql(obj, mon, day)
  })
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
/*
function readDataFromJSON (month) {
  let monthStr = month < 10 ? '0' + month : '' + month;
  let str = fs.readFileSync('./baike.history.today/' + monthStr + '.json', 'utf8');
  saveData(str, month);
}
*/
_.chain(1).range(13).each(queryData).value();
// queryData(3);
/*

let sql = connection.query('select count(*) from history_test', (err, res) => {
  if (err) {
    console.log(err)
  }
  connection.end();
});
console.log(sql.sql);
*/

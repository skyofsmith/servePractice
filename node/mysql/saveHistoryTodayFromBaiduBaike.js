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
// connection.connect();
function html2text(html) {
  let div = document.createElement('div');
  div.innerHTML = html;
  return div.innerText;
}
function saveData2mysql (obj, m, d) {
  console.log(m, obj)
}
function readDataFromJSON (month) {
  let monthStr = month < 10 ? '0' + month : '' + month;
  let str = fs.readFileSync('./baike.history.today/' + monthStr + '.json', 'utf8');
  let result = null;
  try {
    result = JSON.parse(str);
  } catch (e) {
    console.log(e);
  }
  let arr = result[monthStr];
  console.log(arr);
  _.each(arr, (obj, key) => {
    let [mon, day] = key.match(/\d{2}/g);
    saveData2mysql(obj, mon, day)
  })
}
// _.chain(1).range(13).each(readDataFromJSON).value();
readDataFromJSON(11);
/*

let sql = connection.query('select count(*) from history_test', (err, res) => {
  if (err) {
    console.log(err)
  }
  console.log(res);
  connection.end();
});
console.log(sql.sql);
*/

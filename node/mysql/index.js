const mysql = require('mysql');
const http = require('http');
const _ = require('lodash');
const connection = mysql.createConnection({
  host: 'localhost',
  port: 3306,
  user: 'root',
  password: 'root',
  database: 'test'
});
connection.connect();

const monthes = _.chain(1).range(13).map(n => {
  return n < 10 ? '0' + n : '' + n
}).value();
const days = _.chain(1).range(31).map(n => {
  return n < 10 ? '0' + n : '' + n
}).value();
/*

const dayOfYearLinkedList = [];
_.each(monthes, m => {
    _.each(days, d => {
        let item = {
            m: m,
            d: d
        };
        let pre = _.last(dayOfYearLinkedList);
        if (pre) {
            pre.next = item
        }
        dayOfYearLinkedList.push(item)
    });
});
console.log(dayOfYearLinkedList);
*/
_.each(monthes, m => {
  _.each(days, d => {
    getData(m, d)
  });
});

function getData(m, d) {
  const api = 'http://apicloud.mob.com/appstore/history/query?key=2aabaad5867ee&day=';
  http.get(api + m + d, function (res) {
    let result = '';
    res.on('data', (s) => {
      result = result + s;
    });
    res.on('end', () => {
      try {
        let obj = JSON.parse(result);
        if (obj.retCode === '200') {
          _.map(obj.result, function(info){
            insertIntoTable([info.month, info.day, info.date, info.title, info.event]);
          });
        }
      } catch (e) {
        console.error(e)
      }
    });
  }).on('error', (e) => {
    console.log(e)
  });
}

function queryData () {
  connection.query('select * from history_test', function (err, result) {
    if (err) {
      console.log(err)
    }
    console.log(result)
  });
}

function insertIntoTable(datas) {
  connection.query('INSERT INTO history_of_today SET month = ?, day = ?, date = ?, title = ?, event = ?', datas, function (err, result) {
    if (err) {
      console.log(err)
    }
    console.log(result)
  });
}

function insertData(datas) {
  connection.query('INSERT INTO history_test SET month = ?, day = ?, result = ?', datas, function (err, result) {
    if (err) {
      console.log(err)
    }
    console.log(result)
  });
}

// connection.end();

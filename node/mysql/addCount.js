require("babel-core/register");
const mysql = require('mysql');
const _ = require('lodash');
let NUMBER_COUNTS = 30000;
const connection = mysql.createConnection({
  host: 'localhost',
  port: 3306,
  user: 'root',
  password: 'root',
  database: 'test'
});
connection.connect();

function createRandom() {
  return Math.floor(Math.random() * 2147483647)
}

async function main() {
  console.time('queryCount');
  let countRes = await query('select count(*) as count from count');
  console.log(countRes[0].count);
  console.timeEnd('queryCount');

  console.time('query');
  let queryRes = await query('select * from count');
  console.log(queryRes);
  console.timeEnd('query');

  console.time('insert');
  let id = countRes[0].count;
  for (let i = 0; i < NUMBER_COUNTS; i++) {
    await insertRow(id++, createRandom());
  }
  console.timeEnd('insert');


  console.time('query');
  let queryRes2 = await query('select * from count');
  console.log(queryRes2);
  console.timeEnd('query');

  await connection.end();
}
async function query (sql, arr) {
  return new Promise((resolve, reject) => {
    connection.query(sql, arr, (err, res) => {
      if (err) {
        reject(err)
      }
      resolve(res)
    })
  })
}
async function insertRow(id, count) {
  return new Promise((resolve, reject) => {
    connection.query(`insert into count values(?, ?)`, [id, count], (err, res) => {
      if (err) {
        reject(err)
      }
      resolve(res)
    })
  })
}
main();
/*
* queryCount: 19.549ms
* query: 0.884ms
* insert: 66001.529ms
* query: 58.154ms
* */
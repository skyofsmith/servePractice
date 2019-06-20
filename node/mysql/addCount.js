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
  connection.query('select count(*) as count from count', (err, res) => {
    if (err) {
      return
    }
    console.log(res[0].count);
    console.timeEnd('queryCount');
  });
  console.time('query');
  connection.query('select * from count', (err, res) => {
    if (err) {
      return
    }
    console.timeEnd('query');
  });
  // console.time('insert');
  // let id = 0;
  // for (let i = 0; i < NUMBER_COUNTS; i++) {
  //   await connection.query(`insert into count values(?, ?)`, [id++, createRandom()], (err, res) => {
  //     if (err) {
  //       return
  //     }
  //   });
  // }
  // console.timeEnd('insert');
  // await connection.end();
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

// main();
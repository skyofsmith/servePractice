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

  console.time('delete');
  await deleteAll();
  console.timeEnd('delete');

  console.time('queryCount');
  await query('select count(*) as count from count');
  console.timeEnd('queryCount');

  console.time('query');
  await query('select * from count');
  console.timeEnd('query');

  console.time('insert');
  let id = 0;
  for (let i = 0; i < NUMBER_COUNTS; i++) {
    await insertRow(id++, createRandom());
  }
  console.timeEnd('insert');

  console.time('query');
  await query('select * from count');
  console.timeEnd('query');

  console.time('query');
  await query('select * from count');
  console.timeEnd('query');

  console.time('update');
  await updateAll();
  console.timeEnd('update');

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
async function updateAll() {
  return new Promise((resolve, reject) => {
    connection.query(`update count set count = count + 1`, (err, res) => {
      if (err) {
        reject(err)
      }
      resolve(res)
    })
  })
}
async function deleteAll() {
  return new Promise((resolve, reject) => {
    connection.query(`delete from count`, (err, res) => {
      if (err) {
        reject(err)
      }
      resolve(res)
    })
  })
}
main();
/*
* delete: 16.523ms
* queryCount: 1.879ms
* query: 0.556ms
* insert: 63795.645ms
* query: 86.661ms
* query: 28.085ms
* update: 168.973ms
* */
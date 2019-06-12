require("babel-core/register");
const mysql = require('mysql');
const connection = mysql.createConnection({
  host: 'localhost',
  port: 3306,
  user: 'root',
  password: 'root',
  database: 'test'
});
connection.connect();

const qRes = connection.query('select * from user', (err, res) => {
  console.log(err, res)
})
console.log(qRes.sql);
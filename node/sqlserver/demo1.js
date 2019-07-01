require('babel-core/register');
const sql = require('mssql');
const Mock = require('mockjs');
const Random = Mock.Random;
const config = {
  user: 'sa',
  password: 'root',
  server: 'localhost',
  database: 'test',
  options: {
    encrypt: true //使用windows azure，需要设置次配置。
  }
};
const AMOUNT = 30000;

(async function () {
  try {
    let pool = await sql.connect(config);
    console.time('delete');
    let selectRes = await pool.request().query('DELETE FROM [USER]');
    console.log(selectRes);
    console.timeEnd('delete');

    console.time('insert');
    for (let i = 0; i < AMOUNT; i++) {
      await pool.request()
        .input('id', sql.Int, i)
        .input('name', sql.NVarChar, Random.name())
        .input('age', sql.Int, Random.natural(20, 45))
        .query('INSERT INTO [USER] VALUES(@id, @name, @age)');
      // console.log(insertRes);
    }
    console.timeEnd('insert');

    console.time('count');
    let countRes = await pool.request()
      .query('SELECT count(*) AS count From [user]');

    let count = countRes.recordset[0].count;
    console.dir(countRes, count);
    console.timeEnd('count');

    console.time('query');
    let queryRes = await pool.request()
      .query('SELECT * From [user]');
    console.dir(queryRes);
    console.timeEnd('query');

    await pool.close()
  } catch (err) {
    console.log(err)
  }
})();

sql.on('error', err => {
  console.log(err)
});

/*
* JS Data Type To SQL Data Type Map
String -> sql.NVarChar
Number -> sql.Int
Boolean -> sql.Bit
Date -> sql.DateTime
Buffer -> sql.VarBinary
sql.Table -> sql.TVP
*
*
* */

// delete: 181.397ms
// insert: 19793.614ms
// count: 5.563ms
// query: 299.698ms
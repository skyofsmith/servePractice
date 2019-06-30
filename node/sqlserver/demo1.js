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

(async function () {
  try {
    let pool = await sql.connect(config);
    let countRes = await pool.request()
      // .input('input_parameter', sql.Int, -1)
      .query('SELECT count(*) AS count From [user]');

    let count = countRes.recordset[0].count;
    console.dir(countRes, count);

    let insertRes = await pool.request()
      .input('id', sql.Int, 2)
      .input('name', sql.NVarChar, 'sam')
      .input('age', sql.Int, 29)
      .query('insert into [user] values(@id, @name, @age)');

    console.log(insertRes);

    // Stored procedure

    // let result2 = await pool.request()
    //   .input('input_parameter', sql.Int, -1)
    //   .output('output_parameter', sql.VarChar(50))
    //   .execute('procedure_name')
    //
    // console.dir(result2)
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
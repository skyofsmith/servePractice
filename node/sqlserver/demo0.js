require('babel-core/register');
const sql = require('mssql');

const config = {
  user: 'sa',
  password: 'root',
  server: 'localhost',
  database: 'test',
  options: {
    encrypt: true //使用windows azure，需要设置次配置。
  }
};
async function main () {
  
  await sql.connect(config);
  let res = sql.query`select * from [user]`;
  console.log(res)
}
main();
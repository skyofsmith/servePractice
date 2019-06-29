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

sql.connect(config).then(() => {
  return sql.query`select * from [user]`
}).then(result => {
  console.log(result);

}).catch(err => {
  console.error(err);
});

sql.on('error', err => {
  console.error(err);
});
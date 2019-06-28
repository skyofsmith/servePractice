const sql = require('mssql');

const config = {
  user: 'sa',
  password: 'root',
  server: 'localhost',
  database: 'test'
};

sql.connect(config).then(() => {
  return sql.query`select * from [user]`
}).then(result => {
  console.dir(result);

}).catch(err => {
  console.error(err);
});

sql.on('error', err => {
  console.error(err);
});
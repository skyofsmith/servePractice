const sql = require('mssql');
const db = {};
const config = {
  user: 'sa',
  password: 'root',
  server: 'localhost',
  database: 'test'
};

// async () => {
//   try {
//     await sql.connect('mssql://sa:root@localhost/test')
//     // select * from user
//     const result = await sql.query(`SELECT TOP (200) id, name, age FROM [user]`)
//     console.dir(result)
//   } catch (err) {
//     console.log(err)
//   }
// }

sql.connect('mssql://sa:root@localhost/test', res => {
  console.log(res)
  console.log(arguments)
})
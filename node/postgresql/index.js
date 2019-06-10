require("babel-core/register");
const {Client} = require('pg');
const conString = "postgres://postgres:root@localhost/test";

//this initializes a connection pool
//it will keep idle connections open for a (configurable) 30 seconds
//and set a limit of 20 (also configurable)
async function main() {
  let client = new Client(conString);
  await client.connect();
  const {rows: qRes} = await client.query('select * from public.user order by id');
  console.debug(qRes.rows);
  let len = qRes.length + 1;
  let iRes = await client.query(`insert into public.user(id, name, age) values(${len}, '${len}th', 28)`);
  console.log(iRes);
  let uRes = await client.query(`update public.user set age = 29 where id = ${len}`);
  console.log(uRes);
  let dRes = await client.query(`delete from public.user where id = ${len}`);
  console.log(dRes);
  await client.end();
}

main();
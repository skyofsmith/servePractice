require("babel-core/register");
const {Client} = require('pg');
const conString = "postgres://postgres:root@localhost/test";
const NUMBER_COUNTS = 30000;

//this initializes a connection pool
//it will keep idle connections open for a (configurable) 30 seconds
//and set a limit of 20 (also configurable)
function createRandom () {
  return Math.floor(Math.random() * 2147483647)
}
async function main() {
  let client = new Client(conString);
  await client.connect();

  console.time('delete');
  await client.query('delete from count');
  console.timeEnd('delete');

  console.time('query');
  await client.query('select * from count');
  console.timeEnd('query');

  console.time('queryCount');
  const {rows: qRes} = await client.query('select count(*) from count');
  console.timeEnd('queryCount');

  let count = qRes[0].count;
  console.time('insert');
  for (let i = 0; i < NUMBER_COUNTS; i++) {
    await client.query(`insert into count(id, count) values($1, $2)`, [count++, createRandom()]);
  }
  console.timeEnd('insert');

  console.time('update');
  await client.query('update count set count = count + 1');
  console.timeEnd('update');

  await client.end();
}

main();
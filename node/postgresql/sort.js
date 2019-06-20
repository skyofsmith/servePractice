require("babel-core/register");
const {Client} = require('pg');
const conString = "postgres://postgres:root@localhost/test";
const NUMBER_COUNTS = 30000;

async function main() {
  let client = new Client(conString);
  await client.connect();
  console.time('query');
  const {rows: qRes} = await client.query('select * from count order by count');
  console.debug(qRes);
  console.timeEnd('query'); //insert: 102.637ms
  await client.end();
}

main();
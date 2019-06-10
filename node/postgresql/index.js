const pg = require('pg');
const conString = "postgres://postgres:root@localhost/test";

//this initializes a connection pool
//it will keep idle connections open for a (configurable) 30 seconds
//and set a limit of 20 (also configurable)
let client = new pg.Client(conString);
client.connect(function(err) {
  if(err) {
    return console.error('error fetching client from pool', err);
  }
  client.query('select * from public.user', function(err, result) {
    //call `done()` to release the client back to the pool
    // pg.end();
    if(err) {
      return console.error('error running query', err);
    }
    console.debug(result.rows);
    let len = result.rows.length + 1
    client.query(`insert into public.user(id, name, age) values(${len}, '${len}th', 28)`, (err, result) => {
      if(err) {
        return console.error('error running query', err);
      }
      console.debug(result);
      client.query(`update public.user set age = 29 where id = 1`).then((err, result) => {
        if(err) {
          return console.error('error running query', err);
        }
        console.debug(result);
      })
    });
  });
});
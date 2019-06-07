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
  client.query('select 1 + 1 as res', function(err, result) {
    //call `done()` to release the client back to the pool
    // pg.end();
    if(err) {
      return console.error('error running query', err);
    }
    console.log(result.rows[0]);
    //output: 1
  });
});
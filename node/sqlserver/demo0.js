const mssql = require('mssql');
const db = {};

const config = {
  user: 'sa',
  password: 'root',
  server: 'localhost',
  database: 'test'
};
db.sql = function (sql, callBack) {

  var connection = new mssql.Connection(config, function (err) {

    if (err) {

      console.log(err);

      return;

    }

    var ps = new mssql.PreparedStatement(connection);

    ps.prepare(sql, function (err) {

      if (err) {

        console.log(err);

        return;

      }

      ps.execute('', function (err, result) {

        if (err) {

          console.log(err);

          return;

        }


        ps.unprepare(function (err) {

          if (err) {

            console.log(err);

            callback(err, null);

            return;

          }

          callBack(err, result);

        });

      });

    });

  });

};
db.sql('select * from user',function(err,result){

  if (err) {

    console.log(err);

    return;

  }

  console.log('user:',result);

});
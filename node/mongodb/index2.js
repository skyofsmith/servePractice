const MongoClient = require('mongodb').MongoClient;
const assert  =require('assert');
const url = 'mongodb://localhost:27017';
const dbName = 'mydata';
const collectionName = 'lunar';
const solarlunar = require('solarlunar');

MongoClient.connect(url, {useNewUrlParser: true}, function (err, client) {
  assert.equal(null, err);
  console.log("Connected successfully to server");

  const db = client.db(dbName);
  const collection = db.collection(collectionName);
  for (let m = 1; m <= 12; m++) {
    for (let d = 1; d <= 31; d++) {
      let res = solarlunar.solar2lunar(2019, m, d);
      collection.insertOne({
        year: 2019,
        month: m,
        date: d,
        lunarYear: res.lYear,
        lunarMonth: res.lMonth,
        lunarDay: res.lDay
      })
    }
  }
  client.close();

});
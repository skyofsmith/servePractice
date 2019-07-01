require('babel-core/register');
const MongoClient = require('mongodb').MongoClient;
const Mock = require('mockjs');
const Random = Mock.Random;

// Connection URL
const url = 'mongodb://localhost:27017';
const AMOUNT = 30000;
// Database Name
const dbName = 'test';
const collectionName = 'count';

async function main() {
  try {

    let client = await getClient(url);
    let db = client.db(dbName);
    let collection = db.collection(collectionName);

    console.time('remove');
    await remove(collection);
    console.timeEnd('remove');

    console.time('find0');
    await find(collection);
    console.timeEnd('find0');

    console.time('insert');
    let res = [];
    for (let i = 0; i < AMOUNT; i++) {
      res.push({
        name: Random.name(),
        age: Random.natural(20, 45)
      })
    }
    await insert(collection, res);
    console.timeEnd('insert');

    console.time('update');
    await update(collection, {count: 1}, {count: 0});
    console.timeEnd('update');


    console.time('find');
    await find(collection);
    console.timeEnd('find');

    await client.close();

  } catch (e) {
    console.log(e);
    await client.close();
  }
}

main();
// remove: 7.068ms
// find0: 250.888ms
// insert: 427.607ms
// update: 38.330ms
// find: 336.604ms

async function getClient() {
  return new Promise((resolve, reject) => {
    MongoClient.connect(url, {useNewUrlParser: true}, function (err, client) {
      if (err) {
        reject(err);
      }
      resolve(client);
    })
  })
}

async function insert(collection, datas) {
  return new Promise((resolve, reject) => {
    collection.insertMany([].concat(datas), function (err, result) {
      if (err) {
        reject(err);
      }
      resolve(result);
    });
  })
}

async function find(collection, filter) {
  return new Promise((resolve, reject) => {
    collection.find(filter || {}).toArray(function (err, docs) {
      if (err) {
        reject(err);
      }
      resolve(docs);
    })
  })
}

async function update(collection, oldValue, newValue) {
  return new Promise((resolve, reject) => {
    collection.updateOne(oldValue || {}
      , {$set: newValue}, function (err, result) {
        if (err) {
          reject(err);
        }
        resolve(result);
      });
  })
}

async function remove(collection, data) {
  return new Promise((resolve, reject) => {
    collection.deleteOne(data || {}, function (err, result) {
      if (err) {
        reject(err);
      }
      resolve(result);
    });
  })
}

async function index(collection, data) {
  return new Promise((resolve, reject) => {
    collection.createIndex(data, null, function (err, result) {
      if (err) {
        reject(err);
      }
      resolve(result);
    });
  })
}
require('babel-core/register')
const MongoClient = require('mongodb').MongoClient;
const assert = require('assert');

// Connection URL
const url = 'mongodb://localhost:27017';

// Database Name
const dbName = 'test';
const collectionName = 'count';

async function main() {
  let client = await getClient(url);
  let db = client.db(dbName);
  let collection = db.collection(collectionName);
  console.log(await find(collection));
  console.log(await insert(collection, [{count: 1},{count: 2},{count: 3}]));
  console.log(await find(collection));
  console.log(await update(collection,{count: 1}, {count: 0}));
  console.log(await find(collection));
  console.log(await remove(collection, {count: 3}));
  console.log(await find(collection));
  await client.close();
}

main();

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
    collection.updateOne(oldValue
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
    collection.deleteOne(data, function (err, result) {
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
const redis = require("redis");
const client = redis.createClient('6379', '127.0.0.1');

// if you'd like to select database 3, instead of 0 (default), call
// client.select(3, function() { /* ... */ });

client.on("error", function (err) {
  console.log("Error " + err);
});

client.set("name", "sam", redis.print);
// console.log('name is:', );
client.get("name", redis.print);
client.get("name", (err, res) => {
  console.log(err, res)
});
client.hset("hash key", "hashtest 1", "some value", redis.print);
client.hset(["hash key", "hashtest 2", "some other value"], redis.print);
client.hkeys("hash key", function (err, replies) {
  console.log(replies.length + " replies:");
  replies.forEach(function (reply, i) {
    console.log("    " + i + ": " + reply);
  });
  client.quit();
});
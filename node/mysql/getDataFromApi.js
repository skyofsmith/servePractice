const http = require('http');
const _ = require('lodash');

const monthes = _.chain(1).range(13).map(n => {
  return n < 10 ? '0' + n : '' + n
}).value();
console.log(monthes);
const days = _.chain(1).range(31).map(n => {
  return n < 10 ? '0' + n : '' + n
}).value();
console.log(days);

const dayOfYearLinkedList = [];
_.each(monthes, m => {
  _.each(days, d => {
    let item = {
      m: m,
      d: d
    };
    let pre = _.last(dayOfYearLinkedList);
    if (pre) {
      pre.next = item
    }
    dayOfYearLinkedList.push(item)
  });
});
console.log(dayOfYearLinkedList);

const finalResult = [];
const api = 'http://apicloud.mob.com/appstore/history/query?key=2aabaad5867ee&day=';

function getData(item) {
  let {m: month, d: day, next} = item;
  http.get(api + month + day, function (res) {
    let result = '';
    res.on('data', (s) => {
      result = result + s;
    });
    res.on('end', () => {
      try {
        // let obj = JSON.parse(result);
        finalResult.push([month * 1, day * 1, result]);
        if (next) {
          getData(next);
        }
        // console.log(obj)
      } catch (e) {
        console.error(e)
      }
    });
  }).on('error', (e) => {
    console.log(e)
  });
}

module.exports = finalResult;

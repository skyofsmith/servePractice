const assert = require('assert');

// 1.assert()
// 格式 assert(value, message);

// 例子
function add (a, b) {
  return a + b;
}

let expected = add(1,2);
assert( expected === 3, '预期1加2等于3');
assert( expected === 4, '预期1加2等于3');


// 2.assert.ok()
// ok是assert方法的另一个名字，与assert方法完全一样。

// > assert === assert.ok
// true


// 3.assert.equal()
// 格式 assert.equal(actual, expected, [message])

// assert.equal(true, value, message);
// 等同于
// assert(value, message);

{
  let expected = add(1, 2);

  // 以下三句效果相同
  assert(expected === 3, '预期1+2等于3');
  assert.ok(expected === 3, '预期1+2等于3');
  assert.equal(expected, 3, '预期1+2等于3');
}
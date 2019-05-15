const assert = require('assert');

// 1.assert()
// 格式 assert(value, message);

function add (a, b) {
  return a + b;
}

{
  let expected = add(1,2);
  assert( expected === 3, '预期1加2等于3');
  assert( expected === 4, '预期1加2等于3');
}


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

// 4.assert.notEqual()
// 格式 assert.notEqual(actual, expected, [message])

{
  let expected = add(1,2);

  // 以下三种写法效果相同
  assert(expected !== 4, '预期不等于4');
  assert.ok(expected !== 4, '预期不等于4');
  assert.notEqual(expected, 4, '预期不等于4');
}

// 5.assert.deepEqual()
// 格式 assert.deepEqual(actual, expected, [message])

{
  const list1 = [1, 2, 3, 4, 5];
  const list2 = [1, 2, 3, 4, 5];

  assert.deepEqual(list1, list2, '预期两个数组应该有相同的属性');

  const person1 = {"name": "john", "age": "21"};
  const person2 = {"name": "john", "age": "21"};

  assert.deepEqual(person1, person2, '预期两个对象应该有相同的属性');
}

// 6.assert.notDeepEqual()
// 格式 assert.notDeepEqual(actual, expected, [message])

{
  const list1 = [1, 2, , 3, 4, 5];
  const list2 = [1, 2, 3, 4, 5];

  assert.notDeepEqual(list1, list2, '预期两个对象不相等');

  const person1 = { "name":"john", "age":"21" };
  const person2 = { "name":"jane", "age":"19" };

  // deepEqual checks the elements in the objects are identical
  assert.notDeepEqual(person1, person2, '预期两个对象不相等');
}

// 7.assert.strictEqual()
// 格式 assert.strictEqual(actual, expected, [message])

{
  assert.strictEqual(1, '1', '预期严格相等');
}

// 8.assert.notStrictEqual()
// 格式 assert.notStrictEqual(actual, expected, [message])

{
  assert.notStrictEqual(1, true, '预期严格不相等');
}

// 9.assert.throws()
// 格式 assert.throws(block, [error], [message])

{
  // 例一，抛出的错误符合某个构造函数
  assert.throws(
    function() {
      throw new Error("Wrong value");
    },
    Error,
    '不符合预期的错误类型'
  );

  // 例二、抛出错误的提示信息符合正则表达式
  assert.throws(
    function() {
      throw new Error("Wrong value");
    },
    /value/,
    '不符合预期的错误类型'
  );

  // 例三、抛出的错误符合自定义函数的校验
  assert.throws(
    function() {
      throw new Error("Wrong value");
    },
    function(err) {
      if ( (err instanceof Error) && /value/.test(err) ) {
        return true;
      }
    },
    '不符合预期的错误类型'
  );
}

// 10.assert.doesNotThrow()
// 格式 assert.doesNotThrow(block, [message])

{
  assert.doesNotThrow(
    function() {
      console.log("Nothing to see here");
    },
    '预期不抛出错误'
  );
}

// 11.assert.ifError()
// 格式 assert.ifError(value)

{
  // 用法
  function sayHello(name, callback) {
    const error = false;
    const str   = "Hello "+name;
    callback(error, str);
  }

  // use the function
  sayHello('World', function(err){
    assert.ifError(err);
    // ...
    console.log('code continue run...')
  })
}

// 12.assert.fail()
// 格式 assert.fail(actual, expected, message, operator)

{
  assert.fail(21, 42, 'Test Failed', '###');
  // AssertionError: Test Failed
  assert.fail(21, 21, 'Test Failed', '###');
  // AssertionError: Test Failed
  assert.fail(21, 42, undefined, '###')
  // AssertionError: 21 ### 42
}
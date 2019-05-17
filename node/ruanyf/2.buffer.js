{
  let bytes = new Buffer(256);
  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = i;
  }

  let end = bytes.slice(240, 256);

  console.log(end[0]);
  end[0] = 0;
  console.log(end[0]);
}
{
  let bytes = new Buffer(8);

  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = i;
  }

  let more = new Buffer(4);
  bytes.copy(more, 0, 4, 8);
  console.log(more[0]);
}

{
  // 参数是整数，指定分配多少个字节内存
  let hello = new Buffer(5);
  console.log(hello.toString());
  // 参数是数组，数组成员必须是整数值
  hello = new Buffer([0x48, 0x65, 0x6c, 0x6c, 0x6f]);
  console.log(hello.toString());

  // 参数是字符串（默认为utf8编码）
  hello = new Buffer('Hello');
  console.log(hello.length);
  console.log(hello.toString());

  // 参数是字符串（不省略编码）
  hello = new Buffer('Hello', 'utf8');

  // 参数是另一个Buffer实例，等同于拷贝后者
  let hello1 = new Buffer('Hello');
  let hello2 = new Buffer(hello1);
  console.log(hello1.toString(), hello2.toString());
}
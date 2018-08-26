#!/bin/bash

#使用变量
your_name=zz
echo $your_name
echo ${your_name}

#只读变量
myUrl="http://www.google.com"
readonly myUrl
#myUrl="http://www.runoob.com"#报错

#删除变量
canRemove = 123
unset canRemove
echo 'canRemove is ' $canRemove

#字符串


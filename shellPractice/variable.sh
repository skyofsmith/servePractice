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
#单引号
str='this is a string'
echo str
#双引号
str="Hello, I know you are \"$your_name\"! \n"
echo str

# 使用双引号拼接
your_name="runoob"
greeting="hello, "$your_name" !"
greeting_1="hello, ${your_name} !"
echo $greeting  $greeting_1

# 使用单引号拼接
greeting_2='hello, '$your_name' !'
greeting_3='hello, ${your_name} !'
echo $greeting_2  $greeting_3

# 获取字符串长度
string="abcd"
echo ${#string} #输出 4
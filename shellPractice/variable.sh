#!/bin/bash

your_name=zz
echo $your_name
echo ${your_name}

myUrl="http://www.google.com"
readonly myUrl
#myUrl="http://www.runoob.com"#报错

canRemove = 123
unset canRemove
echo 'canRemove is ' $canRemove

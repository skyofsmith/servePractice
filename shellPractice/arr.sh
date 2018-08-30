#!/bin/bash

arr1[0]=value0
arr1[1]=value1
arr1[2]=value2
echo ${arr1[*]}

arr2=(A B "C" D)
echo ${arr2[@]}

my_array[0]=A
my_array[1]=B
my_array[2]=C
my_array[3]=D

echo "数组的元素为: ${my_array[*]}"
echo "数组的元素为: ${my_array[@]}"
echo "数组元素个数为: ${#my_array[*]}"
echo "数组元素个数为: ${#my_array[@]}"


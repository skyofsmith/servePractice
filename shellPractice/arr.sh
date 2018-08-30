#!/bin/bash

arr1[0]=value0
arr1[1]=value1
arr1[2]=value2
echo ${arr1[*]}

arr2=(A B "C" D)
echo ${arr2[@]}


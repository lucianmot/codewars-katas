# Python - Find The Parity Outlier
# https://www.codewars.com/kata/5526fc09a1bbd946250002dc/train/python.

def find_outlier(integers):
    sum = 0;
    for i in integers:
        sum += i%2     
    if sum == 1:
        for i in integers:
            if i%2 == 1:
                return i
    else:
        for i in integers:
            if i%2 == 0:
                return i

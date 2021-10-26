# Python - Array.diff
# https://www.codewars.com/kata/523f5d21c841566fde000009/train/python

def array_diff(a, b):
    result = a
    for i in b:
        result = list(filter(lambda j: j != i, result))
        print (result)
    print ('result: ', result)
    return result

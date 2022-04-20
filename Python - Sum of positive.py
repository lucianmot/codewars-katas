# Sum of positive
# https://www.codewars.com/kata/5715eaedb436cf5606000381/train/python  

def positive_sum(arr):
    results = 0
    for item in arr:
        if item >= 0:
            results += item
    return results

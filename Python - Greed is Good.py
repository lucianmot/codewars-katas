# Python - Greed is Good
# https://www.codewars.com/kata/5270d0d18625160ada0000e4/train/python

def score(dice):
    sum = 0
    dict = {
        1: 0,
        2: 0,
        3: 0,
        4: 0,
        5: 0,
        6: 0
    }

    for i in dice:
        dict[i] += 1
    for k, v in dict.items():
        if v >= 3:
            v -= 3
            if k == 1:
                sum += 1000
            else:
                sum += k*100
        
    if dict[1] != 0:
        sum += dict[1]*100
    if dict[5] != 0:
        sum += dict[5]*50
        
    return sum

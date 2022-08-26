# Python - Beeramid
# https://www.codewars.com/kata/51e04f6b544cf3f6550000c1/train/python

def beeramid(bonus, price):
    counter = 0
    
    while bonus >= (counter + 1)*(counter + 1)*price:
        counter += 1
        bonus -= counter*counter*price
        
    return counter

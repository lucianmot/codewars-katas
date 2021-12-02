# Python - Where my anagrams at? 
# https://www.codewars.com/kata/523a86aa4230ebb5420001e1/train/python

from collections import Counter;

def anagrams(word, words):   
    return [w for w in words if Counter(list(w)) == Counter(list(word))]

# Python - The Hashtag Generator
# https://www.codewars.com/kata/52449b062fb80683ec000024/train/python

def generate_hashtag(s):
    return False if s == '' or len(s) > 140 else '#' + ''.join([i.capitalize() for i in s.split(' ')])

# Python - Find the missing letter
# https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/python

def find_missing_letter(chars):
    alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'
]
    x_pos = 0
    
    for x in alphabet:
        if x == chars[0]:
            x_pos = alphabet.index(x)
            
    chars.pop(0)

    for x in chars:
        if x == alphabet[x_pos + 1]:
            x_pos+=1
        else:
            return alphabet[x_pos + 1]

# Python - Stop gninnipS My sdroW! 
# https://www.codewars.com/kata/5264d2b162488dc400000001/train/python
# This is a speed run we are trying to do multiples, so less refactoring

def spin_words(sentence):
    wArr = sentence.split()
    results = []
    for item in wArr:
        if len(item) >= 5:
            results.append(item[::-1])
        else:
            results.append(item)
    return ' '.join(results)

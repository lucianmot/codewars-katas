# Python - The position of a digital string in a infinite digital string
# https://www.codewars.com/kata/582c1092306063791c000c00/train/python

def find_position(string):
    results = ''
    count = 1
    while count < 100001:
        results += str(count)
        count += 1
    return results.find(string)

# while loop daca nu resultat positiv cut last 10-15 chars and rebuild string loop search again

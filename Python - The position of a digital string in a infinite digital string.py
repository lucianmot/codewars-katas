# Python - The position of a digital string in a infinite digital string
# https://www.codewars.com/kata/582c1092306063791c000c00/train/python   

# attempt 1
def find_position(string):
    results = ''
    count = 1
    while count < 100001:
        results += str(count)
        count += 1
    return results.find(string)

# while loop daca nu resultat positiv cut last 10-15 chars and rebuild string loop search again

# attempt 2
def find_position(string):
    results = ''
    count = 1
    countLimit = 1
    indexCount = 15
    capLimit = 0

    while results.find(string) == -1 and capLimit < 5000000:
        indexCount = indexCount + (len(results)-15)
        results = results[-15:]
        countLimit += 100
        while count < countLimit:
            results += str(count)
            count += 1
            capLimit += 1

#     print("this is the index count")
#     print(indexCount)
    results_prime = results.find(string) + indexCount
#     print("this is results prime")
#     print(results_prime)
    return results_prime
    
            
# while return results.find(string) results[-15:]
# results[-15:] toate index si le adaug  la result

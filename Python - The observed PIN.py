# Python - The observed PIN
#https://www.codewars.com/kata/5263c6999e0f40dee200059d/train/python 
#Work in Progress not yet solved brb

import itertools
def get_pins(observed):
    working_list = []
    results = []
    pos_combo = {
        '1' : ['1', '2', '4'],
        '2' : ['1', '2', '3', '5'],
        '3' : ['2', '3', '6'],
        '4' : ['1', '4', '5', '7'],
        '5' : ['2', '4', '5', '6', '8'],
        '6' : ['3', '5', '6', '9'],
        '7' : ['4', '7', '8'],
        '8' : ['0', '5', '7', '8', '9'],
        '9' : ['6', '8', '9'],
        '0' : ['0', '8']
        }
    obv_array = list(observed)
    for x in obv_array:
        working_list.append(pos_combo.get(x))
        
    for y in itertools.product(*working_list):
        z = ''.join(y)
        results.append(z)
    return results

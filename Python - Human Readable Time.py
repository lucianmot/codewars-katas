# kata title: Python - Human Readable Time
# kata link: https://www.codewars.com/kata/52685f7382004e774f0001f7/train/python

def make_readable(seconds):    
    hours = str(int(seconds / 3600)).zfill(2);    
    seconds %= 3600;  
    minutes = str(int(seconds / 60)).zfill(2);    
    seconds %= 60;
    seconds = str(int(seconds)).zfill(2);   
    return str(hours) + ':' + str(minutes) + ':' + str(seconds);
    

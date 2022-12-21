import random
import os
import json

def iter_loop(loop, size):
    i = random.randrange(size,size+5)
    box = [ i for i in list(range(i)) ]
    box2 = [ (i+(i+1)+(i+2) // 3) for i in box ]
    sum_all = sum(box2)
    print(f'Loop: [{loop}], len: [{i}], 10 elements: {box2[:10]}, results sum: [{sum_all}]')

def repeat_test(iter_count, size):
    ''' Repeat test cycle [iter_count] times.
    '''
    for loop in range(iter_count):
        iter_loop(loop=loop, size=size)
    
    return True
        

def lambda_handler(event, context):
    
    iters = int(os.getenv('ITER_COUNT', 5))
    size = 5_000_000
    
    a = repeat_test(iter_count=iters, size=size)

    # TODO implement
    return {
        'statusCode': 200,
        'body': json.dumps('Hello from python Lambda!')
    }

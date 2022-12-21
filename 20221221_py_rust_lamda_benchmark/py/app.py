import random
import os

iters = int(os.getenv('ITER_COUNT', 5))
size = 10_000_000

def repeat_test(iter_count):
    ''' Repeat test cycle [iter_count] times.
    '''
    for loop in range(iter_count):
        i = random.randrange(size,size+5)
        box = [ i for i in list(range(i)) ]
        box2 = [ (i+(i+1)+(i+2) // 3) for i in box ]
        sum_all = sum(box2)
        print(f'Loop: [{loop}], len: [{i}], 10 elements: {box2[:10]}, results sum: [{sum_all}]')

repeat_test(iter_count=iters)

# print(f'First 10 elements: {box2[:10]}')
# print(f'Len of results array : [{len(box2)}]')
# print(f'Sum of all results: [{sum_all}]')
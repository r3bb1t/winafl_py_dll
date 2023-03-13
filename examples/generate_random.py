import random
import string



def main(*args, **kwargs) -> bytes:
    # print('args: ', args, sep='\t')
    # print('kwargs: ', kwargs, sep='\t')
    # return bytes(''.join(random.choices(string.ascii_uppercase + string.digits, k=N)), 'utf-8')
    return bytes(''.join(random.choices(string.ascii_uppercase + string.digits, k=kwargs['len'])), 'utf-8')

if __name__ == '__main__':
    main(123, 321, kwarg='i am kwarg')




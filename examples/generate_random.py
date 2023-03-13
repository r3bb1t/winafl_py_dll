from random import randint

def main(*args, **kwargs) -> bytes:
    length: int = kwargs['len']
    return bytes([randint(0, 255) for _ in range(length)])
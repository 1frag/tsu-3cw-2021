import math
import timeit

import using_pyo3

TEST_NUMBER = 7_177_162_611_713


def using_python(x: int) -> int:
    i = x + 1
    while True:
        for j in range(2, int(math.sqrt(i) + 1)):
            if i % j == 0:
                break
        else:
            return i
        i += 1


def test_time_using_pyo3_next_prime():
    return using_pyo3.next_prime(TEST_NUMBER)


def test_time_using_python_next_prime():
    return using_python(TEST_NUMBER)


if __name__ == '__main__':
    """
    $ python test_time.py 
    pyo3: 17.036589138035197
    pure: 21.197957821015734
    """
    config = {'number': 20, 'globals': globals()}
    print('pyo3:', timeit.timeit('test_time_using_pyo3_next_prime()', **config))
    print('pure:', timeit.timeit('test_time_using_python_next_prime()', **config))

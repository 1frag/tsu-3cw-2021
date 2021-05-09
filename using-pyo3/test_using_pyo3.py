import pytest
import using_pyo3


@pytest.mark.parametrize("inp, out", [
    (1, 2), (3, 5), (25, 29), (100, 101), (5591, 5623),
])
def test_next_prime(inp, out):
    assert using_pyo3.next_prime(inp) == out

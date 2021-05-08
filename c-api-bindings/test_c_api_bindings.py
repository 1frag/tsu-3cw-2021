import pytest
import c_api_bindings


@pytest.mark.parametrize("inp, out", [
    (1, 2), (3, 5), (25, 29), (100, 101)
])
def test_next_prime(inp, out):
    assert c_api_bindings.next_prime(inp) == out

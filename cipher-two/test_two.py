""" importing the two.py code"""
import two
import pytest

def test_encode():
    """Tests the two.encode function"""
    assert two.encode("Hello World") == "11481249678067805698 10695698668367809533"

def test_bad_encode():
    """Tests for bad input"""
    with pytest.raises(two.EncodingException) as e_info:
        two.encode("1234")

def test_decode():
    """Tests the two.decode() function - going to two ways."""
    assert two.decode(two.encode("Hello World")) == "hello world"

def test_bad_decode():
    """Tests for bad input"""
    with pytest.raises(two.DecodingException) as e_info:
        two.decode("1234 5678 9012")

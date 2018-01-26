""" importing the one.py code"""
import one
import pytest

def test_encode():
    """Tests the one.encode function"""
    assert one.encode("Hello World") == "11481249678067805698 10695698668367809533"

def test_bad_encode():
    """Tests for bad input"""
    with pytest.raises(one.EncodingException) as e_info:
        one.encode("1234")

def test_decode():
    """Tests the one.decode() function - going to two ways."""
    assert one.decode(one.encode("Hello World")) == "hello world"

def test_bad_decode():
    """Tests for bad input"""
    with pytest.raises(one.DecodingException) as e_info:
        one.decode("1234 5678 9012")

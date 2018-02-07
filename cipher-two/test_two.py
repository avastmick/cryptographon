""" importing the two.py code"""
import two
import pytest


def test_version():
    """Tests the version"""
    assert two.get_version() == "1.2, 01-29-2018"


def test_create_key_file():
    """Tests the creation of a new keycode file"""
    assert two.create_key_file("test-two") == "test-two_keycode"


def test_encode():
    """Tests the two.encode function"""
    assert two.encode(
        "test_keycode",
        "Hello World") == "18900578042804288297 83098297384004287936"


def test_bad_encode():
    """Tests for bad input"""
    with pytest.raises(two.EncodingException) as e_info:
        two.encode("test_keycode", "1234")


def test_decode():
    """Tests the two.decode() function - going to two ways."""
    assert two.decode("test_keycode", two.encode(
        "test_keycode", "Hello World")) == "hello world"


def test_bad_decode():
    """Tests for bad input"""
    with pytest.raises(two.DecodingException) as e_info:
        two.decode("test_keycode", "1234 5678 9012")


def test_bad_key_file():
    """Tests for no encryption key found"""
    with pytest.raises(two.KeyFileNotFoundException) as e_info:
        two.decode("wrong_keycode",
                   "18900578042804288297 83098297384004287936")
    with pytest.raises(two.KeyFileNotFoundException) as e_info:
        two.encode("wrong_keycode", "Hello World")

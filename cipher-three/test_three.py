""" importing the three.py code"""
import three
import main
import pytest


def test_version():
    """Tests the version"""
    assert three.get_version() == "1.3, 02-07-2018"


def test_create_key_file():
    """Tests the creation of a new keycode file"""
    assert three.create_key_file("test-three") == "test-three_keycode"


def test_encode():
    """Tests the three.encode function"""
    assert three.encode(
        "test3_keycode", "Hello World!"
    ) == "867329260437960514960514180364036524194068180364438195960514789640104968"


def test_bad_encode():
    """Tests for bad input"""
    with pytest.raises(three.EncodingException) as e_info:
        three.encode("test3_keycode", "\xc4 \xc4")  # passing non-ascii chars


def test_decode():
    """Tests the three.decode() function - going to three ways."""
    assert three.decode("test3_keycode",
                        three.encode("test3_keycode",
                                     "Hello World!")) == "Hello World!"


def test_bad_decode():
    """Tests for bad input"""
    with pytest.raises(three.DecodingException) as e_info:
        three.decode("test3_keycode", "1234 5678 9012")


def test_bad_key_file():
    """Tests for no encryption key found"""
    with pytest.raises(three.KeyFileNotFoundException) as e_info:
        three.decode(
            "wrong_keycode",
            "867329260437960514960514180364036524194068180364438195960514789640104968"
        )
    with pytest.raises(three.KeyFileNotFoundException) as e_info:
        three.encode("wrong_keycode", "Hello World!")


# def test_parser():
#     """Tests the main.py arg parser"""
#     assert main.parse_args(['-v']) == """1.3, 02-07-2018"""

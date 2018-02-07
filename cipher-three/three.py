# Each letter has a 4-digit CODE.

# Hold the codes as a dictionary, so they play with it in the code
# Take a param saying to encode or decode
# Take a param with the message
# Interate through the message changing the letters to code, or vice-versa

import json
import os
import random
import string

# Regenerate the hash, so know which version this is:
# echo -n one.py | sha256sum
VERSION = "1.3, 02-07-2018"
# The length of the code per letter
CODE_LEN = 6


class EncodingException(Exception):
    """Raise when bad values are passed for encoding"""


class DecodingException(Exception):
    """Raise when bad values are passed for encoding"""


class KeyFileNotFoundException(Exception):
    """Raise when no encryption key file found"""


def read_key_file(_keyfile):
    """Reads an encryption file for look up"""
    try:
        with open(_keyfile) as _keycodes:
            key_codes = json.loads(_keycodes.read())
    except:
        raise KeyFileNotFoundException(
            "No encryption key file found. Cannot proceed.")

    return key_codes


def create_key_file(_keyfile):
    """Creates a new key file - note there is NO checking for collisions!!!"""
    lexicon = string.printable
    keycodes = {}
    # Create new codes, each with an lexicon key
    count = 0
    codes = set()
    while count < len(lexicon):
        # Loop here and check for collisions
        code = ''.join(map(str, random.sample(range(10), CODE_LEN)))
        if code not in codes:
            codes.update(code)
            keycodes[lexicon[count]] = code
            count += 1

    new_keyfile = _keyfile + "_keycode"
    keyfile = open(new_keyfile, "w")
    keyfile.write(json.dumps(keycodes))

    return new_keyfile


def encode(_keyfile, _msg):
    """
    Encodes a given message

    @param _msg: The message to encode
    """
    key_codes = read_key_file(_keyfile)
    print("Encoding: \"" + _msg + "\"")
    output = ""
    for c in _msg:
        if c in key_codes:
            output += key_codes[c]
        elif c == ' ':
            output += ' '
        else:
            raise EncodingException(
                "Encoding failed! Please use only lexiconbetical values!")

    return output


def decode(_keyfile, _secret):
    """
    Decodes a given message

    @param _secret: The code to decode
    """
    print("Decoding: \"" + _secret + "\"")
    output = ""
    code = ""
    for c in _secret:
        if len(code) < CODE_LEN:
            code += c
            if len(code) == CODE_LEN:
                output += find_key(_keyfile, code)
                code = ""
    return output


# Below this line is just stuff to make the program easier to use
def find_key(_keyfile, val):
    """return the key of CODE dictionary given the value"""
    key_codes = read_key_file(_keyfile)
    key = ""
    for k, v in key_codes.items():
        if v == val:
            key = k
    if key == "":
        raise DecodingException("Nothing to decode. Bad code!")
    else:
        return key


def get_version():
    """returns the version string"""
    return VERSION

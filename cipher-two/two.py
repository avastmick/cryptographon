# Each letter has a 4-digit CODE.

# Hold the codes as a dictionary, so they play with it in the code
# Take a param saying to encode or decode
# Take a param with the message
# Interate through the message changing the letters to code, or vice-versa

import argparse
import json
import os
import random

# Regenerate the hash, so know which version this is:
# echo -n one.py | sha256sum
VERSION = "1.2, 01-29-2018"
# The length of the code per letter
CODE_LEN = 4

class EncodingException(Exception):
    """Raise when bad values are passed for encoding"""

class DecodingException(Exception):
    """Raise when bad values are passed for encoding"""

class KeyFileNotFoundException(Exception):
    """Raise when no encryption key file found"""

def read_key_file(_keyfile):
    """Reads an encryption file for look up"""
    if os.path.isfile(_keyfile):
        with open(_keyfile) as _keycodes:
            key_codes = json.load(_keycodes)
    else:
        raise KeyFileNotFoundException("No encryption key file found. Cannot proceed.")

    return key_codes

def create_key_file(_keyfile):
    """Creates a new key file - note there is NO checking for collisions!!!"""
    alpha = "abcdefghijklmnopqrstuvwxyz"

    # Create new codes, each with an alpha key
    codes = {c: ''.join(map(str, random.sample(range(10), 4))) for c in alpha}

    new_keyfile = _keyfile + "_keycode"
    keyfile = open(new_keyfile, "w")
    keyfile.write(json.dumps(codes))

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
        if c.lower() in key_codes:
            output += key_codes[c.lower()]
        elif c == ' ':
            output += ' '
        else:
            raise EncodingException("Encoding failed! Please use only alphabetical values!")

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
        if c == " ":
            output += c
            code = ""
        elif len(code) < 4:
            code += c
            if len(code) == 4:
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
    """Print the version of the code"""
    return VERSION

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Encodes or decodes secret messages")
    parser.add_argument('action',
                        help="States whether the message should be encoded or decoded, or whether a key should be created.",
                        choices=["encode", "decode", "new"])
    parser.add_argument('keyfile', help="The encryption key file to use, or the name of a new key file (e.g. alice-bob)")
    parser.add_argument('message', nargs='?', help="The message to encode / decode, (must be in \"quotes\")")
    parser.add_argument('-v', '--version', help="Returns the sha256 hash for this code", action='version', version=get_version())

    args = parser.parse_args()
    if args.action == "version":
        print(get_version())
    elif args.action == "decode":
        print("Message: " + decode(args.keyfile, args.message))
    elif args.action == "encode":
        print("Code: " + encode(args.keyfile, args.message))
    elif args.action == "new":
        print("New keycode file: " + create_key_file(args.keyfile))

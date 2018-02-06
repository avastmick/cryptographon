# Each letter has a 4-digit CODE.

# Hold the codes as a dictionary, so they play with it in the code
# Take a param saying to encode or decode
# Take a param with the message
# Interate through the message changing the letters to code, or vice-versa

# Regenerate the hash, so know which version this is:
# echo -n one.py | sha256sum
VERSION = "1.0, 01-18-2018"
# The length of the code per letter
CODE_LEN = 4
# A lookup of codes
CODE = {
    "a" : "6520",
    "b" : "2143",
    "c" : "3990",
    "d" : "9533",
    "e" : "1249",
    "f" : "8942",
    "g" : "1043",
    "h" : "1148",
    "i" : "2397",
    "j" : "7753",
    "k" : "6521",
    "l" : "6780",
    "m" : "0067",
    "n" : "1258",
    "o" : "5698",
    "p" : "9901",
    "q" : "9806",
    "r" : "6683",
    "s" : "6799",
    "t" : "5320",
    "u" : "3118",
    "v" : "2679",
    "w" : "1069",
    "x" : "9001",
    "y" : "5477",
    "z" : "9900"
}

class EncodingException(Exception):
    """Raise when bad values are passed for encoding"""


class DecodingException(Exception):
    """Raise when bad values are passed for encoding"""

def encode(_msg):
    """
    Encodes a given message

    @param _msg: The message to encode
    """
    print("Encoding: \"" + _msg + "\"")
    output = ""
    for c in _msg:
        if c.lower() in CODE:
            output += CODE[c.lower()] 
        elif c == ' ':
            output += ' '
        else:
            raise EncodingException("Encoding failed! Please use only alphabetical values!")
        
    return output

def decode(_code):
    """
    Decodes a given message

    @param _code: The code to decode
    """
    print("Decoding: \"" + _code + "\"")
    output = ""
    code = ""
    for c in _code:
        if c == " ":
            output += c
            code = ""
        elif len(code) < 4:
            code += c
            if len(code) == 4:
                output += find_key(code)
                code = ""
    return output


# Below this line is just stuff to make the program easier to use
def find_key(val):
    """return the key of CODE dictionary given the value"""
    key = ""
    for k, v in CODE.items():
        if v == val:
            key = k
    if key == "":
        raise DecodingException("Nothing to decode. Bad code!")
    else:
        return key

def get_version():
    """returns the version string"""
    print(VERSION)

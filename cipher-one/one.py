# Each letter has a 4-digit CODE.

# Hold the codes as a dictionary, so they play with it in the code
# Take a param saying to encode or decode
# Take a param with the message
# Interate through the message changing the letters to code, or vice-versa

import argparse

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
        else:
            output += ' '
        
    print("Code: " + output)

def decode(_msg):
    """
    Decodes a given message

    @param _msg: The code to decode
    """
    print("Decoding: \"" + _msg + "\"")
    output = ""
    code = ""
    for c in _msg:
        if c == " ":
            output += c
            code = ""
        elif len(code) < 4:
            code += c
            if len(code) == 4:
                output += find_key(code)
                code = ""

    print("Message: " + output)




# Below this line is just stuff to make the program easier to use
def find_key(val):
    """return the key of CODE dictionary given the value"""
    return [k for k, v in CODE.iteritems() if v == val][0]

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Encodes or decodes secret messages")
    parser.add_argument('action',
                        help="States whether the message should be encoded or decoded",
                        choices=["encode", "decode"])
    parser.add_argument('message', help="The message to encode / decode, (must be in \"quotes\")")

    args = parser.parse_args()

    if args.action == "encode":
        encode(args.message)
    elif args.action == "decode":
        decode(args.message)

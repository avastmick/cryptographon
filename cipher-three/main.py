import sys
import argparse
import three


def parse_args(args):
    parser = argparse.ArgumentParser(
        description="Encodes or decodes secret messages")
    parser.add_argument(
        'action',
        help=
        "States whether the message should be encoded or decoded, or whether a key should be created.",
        choices=["encode", "decode", "new"])
    parser.add_argument(
        'keyfile',
        help=
        "The encryption key file to use, or the name of a new key file (e.g. alice-bob)"
    )
    parser.add_argument(
        'message',
        nargs='?',
        help="The message to encode / decode, (must be in \"quotes\")")
    parser.add_argument(
        '-v',
        '--version',
        help="Returns the version of this code",
        action='version',
        version=three.get_version())
    return parser.parse_args(args)


if __name__ == '__main__':
    args = parse_args(sys.argv[1:])
    if args.action == "version":
        print(three.get_version())
    elif args.action == "decode":
        print("Message: " + three.decode(args.keyfile, args.message))
    elif args.action == "encode":
        print("Code: " + three.encode(args.keyfile, args.message))
    elif args.action == "new":
        print("New keycode file: " + three.create_key_file(args.keyfile))

import argparse
import one

if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description="Encodes or decodes secret messages")
    parser.add_argument(
        'action',
        help=
        "States whether the message should be encoded or decoded, or whether a key should be created.",
        choices=["encode", "decode"])
    parser.add_argument(
        'message',
        nargs='?',
        help="The message to encode / decode, (must be in \"quotes\")")
    parser.add_argument(
        '-v',
        '--version',
        help="Returns the sha256 hash for this code",
        action='version',
        version=one.get_version())

    args = parser.parse_args()
    if args.action == "version":
        print(one.get_version())
    elif args.action == "decode":
        print("Message: " + one.decode(args.message))
    elif args.action == "encode":
        print("Code: " + one.encode(args.message))

# Cipher One

The ``cipher-one`` code creates a cipher based on the girls original paper code book.

<img src="../img/original.jpg" alt="The Original" width="250" height="200" />

We turned this paper into mini programs that do exactly what your would do with paper and pen. You need some pretty good computer skills to use them, but you're spies, or something, right!

## Python

The quickest to build this in was going to be [Python](https://www.python.org/).

Python is:

- Easy to write
- Easy to understand / read
- Nice to play with and change

But, we're dealing with secrets here! And Python is in an open text format. Sure we could do something to obfuscate the internals, but nonetheless, Python is an interpreted language and as such can be [simply read or worked out fairly easily](https://stackoverflow.com/questions/261638/how-do-i-protect-python-code).

## Rust

There are other languages out there that are more secure.

[Rust](https://www.rust-lang.org/en-US/) on the other hand is compiled. What does that mean? If means that in order to run it you have to do an extra step and create a binary file using a compiler, in this case ``rustc``. A binary file is hard (but not impossible) to put back into a readable format. So our secrets are more secure.

Compiled languages are also faster to run, and in the case of Rust safer as it applies a lot of code checking with you compile it so you code is less likely to break when run. But, Rust:

- It's hard to write at first
- It's hard to read - it's real computer code, a lot doesn't make sense
- It's difficult to make changes and play with.

## Problems

- What are the problems with ``cipher-one``?
- How can we improve it?
- What else must it do for it to be really secure?

## How do we crack the code?

First, we need to know what the ciphertext looks like and see if we can work out if there are any patterns in in.

Because we can open the code up, say## How do we crack the code? First, we need to know what the ciphertext looks like and see if we can work out if there are any patterns in in. Because we can open the code up, say `one.py`, and see how it works, we could cheat. But, we don't want to, do we?

Let's just look at some ciphertext and see what we can do with it.

```bash

```

This is a fairly long piece of text that should give us some clues as to the underlying plaintext.

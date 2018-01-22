# Cipher Two

There are problems with ``cipher-one``. To understand what they are, let's think of some scenarios.

- Alice wants to send a message to Bob, (it is always [Alice and Bob in cryptography](https://en.wikipedia.org/wiki/Alice_and_Bob), by the way).
- However, Eve wants to know what the message is, (because she likes Bob and is jealous of Alice), Eve is an evesdropper.
- Alice knows this, so uses our ``cipher-one`` tool to **encode** the message.
- Bob also has a copy of the ``cipher-one`` tool and he uses it to **decode**, or decipher, Alice's messages.
- Eve doesn't have the ``cipher-one`` tool so cannot understand the messages Alice and Bob send between each other.
- Alice then wants to send a message to Carol, but she doesn't want *either* Bob or Eve knowing what was sent.
- **Uh oh!** Alice cannot use the ``cipher-one`` tool as Bob will be able to decipher the messages that she wants to send only to Carol!

*Okay, now we have a problem.*

Alice needs to be able to send a secret message to anyone without anyone else being able to intercept the message, decoding and reading it. Bob probably wants to be able to do the same, and Alice, even Eve. In fact everyone may need to send a message to another person without anyone else reading it.

Let's split this out.

 1. The message needs to be secure - we'll get back to this, but for now let's say our code is good enough.
 1. The means of encoding the message needs to be unique to two parties - we need to work on this.

How can we do this.

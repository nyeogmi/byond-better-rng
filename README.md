The problem with Space Station 13 isn't the shonky game design, the bad netcode, or the combination of the shonky game design and the bad netcode.

It's the fact that it uses a terrible RNG.

I mean, not a terrible RNG. It uses the Mersenne Twister, which is, by all accounts, a pretty good RNG. But it's kind of slow and it's not cryptographically safe.

So here's a bridge that lets you use ChaCha8 from BYOND. ChaCha8 is such a good RNG that Zig uses it. Why aren't you using it?

All you have to do is install this auxtools-based extension and then you're safe. Completely safe! Nobody will be able to use elaborate reverse-engineering tools to win big money on the slot machines. Nobody will be able to guess the DRAGON PASSWORD. (as long as the DRAGON PASSWORD is randomly generated) Nobody will be able to prove (using advanced mathematics) that your Ouija board is capable of spelling "ass."

Thanks to Bhijn for code help, Putnam for auxmos (which my implementation is based on), and Willox for auxtools.

PS: If you're from Splurt, you should definitely use this. Why not install it now?

PS 2: Wanna prove it's doing something? Make all the hooks in src/lib.rs return 0.

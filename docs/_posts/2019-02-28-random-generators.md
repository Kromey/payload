---
title: Random Generators
tags: prng rng random
---
It's been quite a while since I decided I would start working on this new project, which I'll introduce more fully later. For this one, however, I've decided to use a language that's new to me: Rust.

Rust has a lot of advantages to it, which I won't go into right now. This post is about how I decided to start getting my feet wet: I wrote a PRNG (pseudorandom number generator). And then I wrote 2 more.

If somehow you're reading this and don't know, roguelike games are dependent upon having a good source of "randomness" to handle everything from dungeon generation to "did your swing actually hit that kobold?" Arguably even more important, though, is that the game has to have *repeatable randomness*, that is given the same environment and the same conditions, your random results should be the same as they were last time. This is most often described as the "world seed", a reference to the seed, or starting point, of the random number generator. Of course, since the output is in fact based on this seed, it's not truly random; what you actually have is a *pseudo*random number generator, or *P*RNG.

Being new to Rust, I decided I'd start this project by writing my own PRNG. Not being a super mathematician (my high school nickname notwithstanding...), however, I wasn't going to try and invent a PRNG algorithm. I was just going to look one up and implement that. And one that I'd heard of recently, with several touted benefits (one of which was ease of implementation), was the [PCG32 family](http://www.pcg-random.org/). Another of its claims to fame is the ability to specify a "stream", purportedly a huge benefit for roguelikes (but we'll come back to this later).

So I dove in. I grabbed the C code for one of the members of the PCG32 family, and I set out to port the code to Rust. Which was, as promised, not hard -- once I realized that I'd glossed over a key aspect of the constructor, resulting in every single generator starting with 0 and then a (relatively) very small number, before seeming to finally get going and producing random numbers. I even downloaded the C code and ran it side-by-side to ensure my implementation matched the source -- and it did!

Unfortunately, due to Rust's behavior with integer types and overflows (specifically, it panics on an overflow in debug mode), my PRNG was just *littered* with casts to and from larger types. Rust was looking a lot less appealing at this point, but I reasoned that this wasn't going to be typical. After all, how often does one deal with characters having more than 4,294,967,295 hit points? (But stick around, because we'll come back to this, too.)

It was right around this point where unrelated circumstances suddenly lead to me learning about another family of PRNGs, Xorshift, and specifically [Xorshift\*](https://en.wikipedia.org/wiki/Xorshift#xorshift*). Touted as being fast -- faster even than PCG32 -- they unfortunately don't produce good quality output without some additional tweaks. In reading about them, however, I then found a related family, the [Xoshiro](http://xoshiro.di.unimi.it/) (XOR, Shift, Rotate) family. They claim to be even faster than Xorshift while not only being stronger, but passing every single statistical test for randomness. And they're not hard to implement either!

So once again I grabbed the C code and started to port that into Rust, choosing the Xoshiro256\*\* generator ("our **all-purpose, rock-solid** generator", as the authors describe it). This time I got it done even quicker, in part because coding the PCG32 generator had taught me a few things, *and* I had fewer casts to higher-order integers than my PCG32 implementation. Then I followed the author's recommendation and also implemented another simple PRNG, [SplitMix64](http://xoshiro.di.unimi.it/splitmix64.c), to take a 64-bit seed and initialize the 256-bit state.

With successful generators in hand, I looked at my code. So many casts! Surely there was a better way? So I looked up the `rand` crate, found its source, and the several choices of generators it had -- including both PCG32 and Xoshiro256\*\*. Looking at their implementations, they were (after de-abstracting the macros) basically identical to my own with one key difference: No casts.

Instead, they take advantage of some Rust methods I didn't know existed: `u64.wrapping_mul()` and the other `wrapping_*()` methods. With these calls, instead of doing "straight" multiplication and addition, the math functions identically to the C implementations, effectively "truncating" (really, "wrapping") any bits that exceed the size of the variable. So a quick change to my generators, and my code is essentially identical to what's in the `rand` crate.

Now with 2 (well, 3, but really 2 usable) PRNGs, which one am I going to use? And am I going to stick with mine instead of the `rand` crate?

To answer that second question first: Yes, I am. Why? Well, because *I* wrote it, that's why! There's also, since I now have this code I've written, no reason to add the dependency on the external `rand` crate. Keeps things a little cleaner, although I suspect in the grand scheme of things that one crate would be a *very* small portion of the dependencies. And I can easily add my helper methods, such as one to roll 3d6 dice for me, directly to it (although I could write a Trait and implement that onto the `rand` crate for the same effect...).

As to the first question: I've grown a little cold on PCG32. For starters, while the multiple streams feature seems great at first glance, once you start to think about the implementation you realize that now you have to track your seed separately plus all these various streams; why not just have a deterministic seed generator for each "stream" instead? All the examples I've seen say things like using `rngcache.get("dungeon-level-1")`, where that string gets hashed to become the "stream" alongside the world's seed. Well, why can't you hash that string with the seed instead?

This feature just doesn't hold up as being truly beneficial.

Another feature touted for PCG32 is its small state: Just a pair of 64-bit integers. Which is half what Xoshiro256\*\*, for example, uses. But: So what? Who out there is using a system where the difference of 128 bits actually matters? 32 Xoshiro256\*\* generators fit in a mere 1 KB of RAM; sure, you can put 64 PCG32s in that same space, but really this is seriously small potatoes we're talking about here! At this point the choice of which image compression algorithm you use for your assets is many orders of magnitude more important!

Secondly, there's some [serious questions about the quality of PCG32](http://pcg.di.unimi.it/pcg.php). I'm not saying I need a perfect PRNG (and what would a "perfect" one look like anyway?), but if Xoshiro256\*\* *is* in fact better at being "random", *and* is faster at the same time as its authors claim, then doesn't that make it the superior PRNG?

So I'm leaning toward using Xoshiro256\*\*.

Still, chiefly because it is so popular, I want to give PCG32 a fair shake. So I'm going to benchmark both of them, just as soon as I figure out how to run benchmarks in Rust. If PCG32 just blows Xoshiro256\*\* out of the water, then I'll use it. If it's only a little bit faster, and especially if it's slower, I'll let my qualms above rule it out.


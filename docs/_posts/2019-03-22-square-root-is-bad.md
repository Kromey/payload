---
title: Square Root is Bad, M'Kay?
tags: algorithms optimization
---
If you've worked on a Computer Science degree or studied computer algorithms in-depth, you've probably encountered the statement that computing square roots is an "expensive" operation. That is, while computers are really fast at multiplying numbers together, they really *suck* at calculating the square root of a number.

I've always just taken this on faith as a truism. Recently, however, I wondered just how true it is -- or even if it's still true on modern computers. So I wrote a quick benchmark in Rust, and found that it takes about 41µs (that's 41 microseconds, if you're not up on your SI prefixes) to calculate 10,000 square roots. That's pretty fast!

...until you benchmark squaring: Calculating the square of 10,000 numbers took a mere 1ns. That's *40,000 times faster* -- you can calculate 40,000 squares in the same amount of time that it takes to find *one* square root!

(Usual caveats apply: This is the timing on my computer, with no attempt to hand-optimize either code (not that there's much optimizing that could be done), with default "release" compiler optimizations. Your results will probably vary. Still, relative differents should remain.)

So that's all interesting and such, but what does it have to do with roguelikes?

Well, if you ever compare distances -- say, testing whether a potential target is within range of your weapon -- it applies a lot, because (unless you're using [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)) computing the distance between two points requires a square root operation: `sqrt((x1 - x2)^2 + (y1 - y2)^2)`

And if you're trying to find out how many entities are within the radius of a fireball or grenade explosion, now you're suffering this cost repeatedly!

Now, there are some easy pruning operations that can be done, such as only testing "true" distance of entities within the equivalent Manhattan distance, but you still wind up doing potentially dozens or more of these seriously expensive square root operations. But what if I told you there's an easy way to go from "dozens" to "*zero*"?

Instead of calculating the distance between two points to compare to a range, square the range and calculate the "distance squared" -- our good friend the Pythagorean Thereom again, except we leave out the square root operation. Two distances A and B satisfy all of the same (in)equality expressions as A^2 and B^2 do, and (as the benchmarks above show) you can find these values *a lot* faster!

So when you're detonating that fireball or testing that bow's range, square the distance and then only calculate the distance squared to each entity. While you're still want to do the same pruning operations for maximum optimization, you're certain to see performance improvements even if you drop the pruning.

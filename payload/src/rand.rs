mod xoshiro;

#[derive(Debug)]
pub struct Rand {
    rng: xoshiro::Xoshiro256,
}

impl Rand {
    /// Create a new PRNG generator.
    pub fn new() -> Rand {
        Rand {
            rng: xoshiro::Xoshiro256::new(),
        }
    }

    /// Create a new PRNG generator from a seed
    pub fn from_seed(seed: u64) -> Rand {
        Rand {
            rng: xoshiro::Xoshiro256::from_seed(seed),
        }
    }

    /// Generate a random 64-bit integer
    pub fn rand_u64(&mut self) -> u64 {
        self.rng.next()
    }

    /// Generate a random 32-bit integer
    pub fn rand_u32(&mut self) -> u32 {
        self.rand_u64() as u32
    }

    /// Generate a random 32-bit integer that's in the range (min, max]
    pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        let range = max - min;

        self.rand_bound(range) + min
    }

    /// Generate a 32-bit integer in the range (0, bound]
    ///
    /// This is a port to Rust of the C `pcg32_boundedrand_r` algorithm found
    /// in the PCG32 reference implementation at
    /// https://github.com/imneme/pcg-c-basic/blob/master/pcg_basic.c
    pub fn rand_bound(&mut self, bound: u32) -> u32 {
        // This is magic math that gives us how many numbers
        // we have to throw out (from our overall range) in
        // order to produce an *unbiased* result in the
        // desired range:
        // (MAX - threshold) % bound == 0
        let threshold = (std::u32::MAX - bound) % bound;

        // Uniformity guarantees that this loop will eventually
        // terminate, and in fact in practice we'll be given
        // small bounds which will mean only a very small slice
        // of our total range will be being ignored; in other
        // words, most of the time this loop will exit on the
        // first iteration.
        let mut n: u32;
        loop {
            n = self.rand_u32();
            if n > threshold {
                break;
            }
        }

        n % bound
    }

    /// Roll a `sides`-sided dice and return the result
    ///
    /// The result will always be an integer in the range [1, sides]
    pub fn roll_dx(&mut self, sides: u32) -> u32 {
        self.rand_bound(sides) + 1
    }

    /// Roll `dice` `sides`-sided dice, and return the sum of the results
    pub fn roll_ndx(&mut self, dice: u32, sides: u32) -> u32 {
        (0..dice).fold(0, |sum, _| sum + self.roll_dx(sides))
    }
}

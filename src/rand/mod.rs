/// Implementation of xoshiro256**, ported into Rust from the C implementation
/// available at http://xoshiro.di.unimi.it
#[derive(Debug)]
pub struct Xoshiro256 ( u64, u64, u64, u64 );

impl Xoshiro256 {
    /// Create a new PRNG generator.
    ///
    /// # TODO
    ///
    /// Currently this method uses a static, hard-coded seed. This should be
    /// changed to instead use a new, arbitrary seed on each run.
    pub fn new() -> Xoshiro256 {
        Xoshiro256::from_seed(0xCAFEBABE)
    }

    /// Create a new PRNG generator from a seed
    pub fn from_seed(seed: u64) -> Xoshiro256 {
        //Per Blackman & Vigna, seed with output of splitmix64
        let mut seeder = SplitMix64::from_seed(seed);
        Xoshiro256 ( seeder.next(), seeder.next(), seeder.next(), seeder.next() )
    }

    /// Generate a random 64-bit integer
    pub fn rand(&mut self) -> u64 {
        let result = self.1.wrapping_mul(5).rotate_left(7).wrapping_mul(9);

        let t = self.1 << 17;
        self.2 ^= self.0;
        self.3 ^= self.1;
        self.1 ^= self.2;
        self.0 ^= self.3;

        self.2 ^= t;

        self.3 = self.3.rotate_left(45);

        result as u64
    }

    /// Generate a random 32-bit integer
    pub fn rand32(&mut self) -> u32 {
        self.rand() as u32
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
            n = self.rand32();
            if n > threshold {
                break;
            }
        }

        n % bound
    }

    /// Roll a `sides`-sided dice and return the result
    ///
    /// The result will always be an integer in the range [0, sides]
    pub fn roll(&mut self, sides: u32) -> u32 {
        self.rand_bound(sides) + 1
    }

    /// Roll `n` `sides`-sided dice, and return the sum of the results
    pub fn nroll(&mut self, dice: u32, sides: u32) -> u32 {
        let mut sum = 0;
        for _i in 0..dice {
            sum += self.roll(sides);
        }

        sum
    }
}

/// The SplitMix64 algorithm is another Rust port of the C reference implementation
/// from http://xoshiro.di.unimi.it/splitmix64.c
///
/// This algorithm is only used to generate the 256 bits of state to initialize
/// Xoshiro256** from a 64-bit seed, which is why it is not pub.
#[derive(Debug)]
struct SplitMix64(u64);

impl SplitMix64 {
    /// Create a new PRNG generator from a seed
    fn from_seed(seed: u64) -> SplitMix64 {
        SplitMix64(seed)
    }

    /// Generate a 64-bit random integer
    fn next(&mut self) -> u64 {
        // We need to do all our math as 128bits to avoid overflow
        // but we then need to drop all of the top 64 bits for correct
        // behavior
        let mut z = self.0.wrapping_add(0x9e3779b97f4a7c15);
        self.0 = z;

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}


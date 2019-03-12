use std::time::SystemTime;

/// Implementation of xoshiro256**, ported into Rust from the C implementation
/// available at http://xoshiro.di.unimi.it
#[derive(Debug)]
pub struct Xoshiro256 ( u64, u64, u64, u64 );

impl Xoshiro256 {
    pub fn new() -> Xoshiro256 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Xoshiro256::from_seed(n.as_secs()),
            Err(_) => panic!("SystemTime failed to give us a time!"),
        }
    }

    /// Create a new PRNG generator from a seed
    ///
    /// Uses SplitMix64 to turn the 64-bit seed into 256 bits of state.
    pub fn from_seed(seed: u64) -> Xoshiro256 {
        //Per Blackman & Vigna, seed with output of splitmix64
        let mut seeder = SplitMix64::from_seed(seed);
        Xoshiro256 ( seeder.next(), seeder.next(), seeder.next(), seeder.next() )
    }

    /// Get the next random number in our sequence
    pub fn next(&mut self) -> u64 {
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

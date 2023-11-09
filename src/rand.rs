pub use bevy_rand::prelude::*;
pub use rand::prelude::*;

type RngAlgorithm = WyRand;

pub type RandPlugin = EntropyPlugin<RngAlgorithm>;
pub type WorldRng = GlobalEntropy<RngAlgorithm>;
pub type RngComponent = EntropyComponent<RngAlgorithm>;

pub fn world_seed<S: std::hash::Hash>(seed: S) -> WorldRng {
    rand_seeder::Seeder::from(seed).make_rng()
}

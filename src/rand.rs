use bevy_rand::prelude::{EntropyPlugin, WyRand};

type RngAlgorithm = WyRand;

pub type RandPlugin = EntropyPlugin<RngAlgorithm>;

pub mod prelude {
    pub use rand::prelude::*;

    use super::RngAlgorithm;
    use bevy_rand::{prelude::EntropyComponent, resource::GlobalEntropy};

    pub type WorldRng = GlobalEntropy<RngAlgorithm>;
    pub type RngComponent = EntropyComponent<RngAlgorithm>;
}

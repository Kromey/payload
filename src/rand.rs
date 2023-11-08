use std::hash::Hash;

use bevy::{ecs::system::SystemParam, prelude::*};
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use rand_seeder::Seeder;

#[derive(Debug, Clone, Copy, Hash, Resource)]
pub struct WorldSeed {
    seed: <ChaChaRng as SeedableRng>::Seed,
}

impl WorldSeed {
    pub fn from_seed<S: Hash>(seed: S) -> Self {
        Self {
            seed: Seeder::from(seed).make_seed(),
        }
    }
}

impl FromWorld for WorldSeed {
    fn from_world(_world: &mut World) -> Self {
        Self {
            seed: ChaChaRng::from_entropy().gen(),
        }
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct LocalRng(ChaChaRng);

#[derive(Debug, SystemParam)]
pub struct SystemRand<'w, 's> {
    world_seed: Option<Res<'w, WorldSeed>>,
    local_rng: Local<'s, Option<LocalRng>>,
}

impl<'w, 's> SystemRand<'w, 's> {
    pub fn rng<S: Hash>(&mut self, local_seed: S) -> &mut LocalRng {
        let world_seed = self
            .world_seed
            .as_ref()
            .expect("Attempted to get local RNG without a world seed");

        if self.local_rng.is_none() {
            *self.local_rng = Some(LocalRng(
                Seeder::from((**world_seed, local_seed)).make_rng(),
            ));
        }

        self.local_rng.as_mut().expect("No local RNG!??")
    }
}

pub fn test_rand(mut local_rng: SystemRand) {
    let rng = local_rng.rng("Local Seed");
    info!("RNG output: {}", rng.gen::<u8>());
}

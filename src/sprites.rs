use bevy::prelude::*;

#[derive(Debug, Default, Clone, Resource)]
pub struct Sprites {
    pub player: Handle<Image>,
}

pub fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprites = Sprites {
        player: asset_server.load("robot.png"),
    };

    commands.insert_resource(sprites);
}

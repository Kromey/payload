use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

use bevy::prelude::*;

use crate::player::Player;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainCamera;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct FollowPlayer;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera, FollowPlayer));
}

/// Update the camera position to follow the player
///
/// # Running this system
///
/// This system needs to run in the `PostUpdate` schedule to ensure any `Transform` updates have
/// been done already, but must run before Bevy propagates `Transform` to `GlobalTransform` to
/// avoid the camera "lagging".
///
/// # Possible future bug?
///
/// This system uses the `Transform` component, which is fine as long as both camera and player
/// have the same parent, or are both parent-less. If camera and/or player are ever parented to
/// a different entity, then relying on `Transform` will no longer work, but neither can we use
/// `GlobalTransform` because that hasn't been updated yet!
///
/// Another potential solution is to child the camera to the player and enforce a `Vec3::ZERO`
/// translation on the camera to keep it on the player; if the camera needs to be moved, or the
/// player needs to be despawned, this relationship can be "broken" at that time and then
/// recreated after.
#[allow(clippy::type_complexity)]
pub fn follow_player(
    mut query_set: ParamSet<(
        Query<&Transform, (With<Player>, Changed<Transform>)>,
        Query<&mut Transform, With<FollowPlayer>>,
    )>,
) {
    // NOTE: Can we use GlobalTransform here instead?
    // Transform has not yet been propagated to GlobalTransform, but if camera and player
    // don't both have the same parent (or if they aren't both parent-less) this will break
    let player_translation = match query_set.p0().get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    for mut camera_transform in query_set.p1().iter_mut() {
        camera_transform.translation = player_translation;
    }
}

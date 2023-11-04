use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainCamera;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Follow(pub Entity);

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

/// Update the camera position to follow an entity
///
/// # Running this system
///
/// This system needs to run in the `PostUpdate` schedule to ensure any `Transform` updates have
/// been done already, but must run before Bevy propagates `Transform` to `GlobalTransform` to
/// avoid the camera "lagging".
///
/// # Possible future bug?
///
/// This system uses the `Transform` component, which is fine as long as both camera and target
/// have the same parent, or are both parent-less. If camera and/or target are ever parented to
/// a different entity, then relying on `Transform` will no longer work, but neither can we use
/// `GlobalTransform` because that hasn't been updated yet!
///
/// Another potential solution is to child the camera to the target and enforce a `Vec3::ZERO`
/// translation on the camera to keep it on the target; if the camera needs to be moved, or the
/// target needs to be despawned, this relationship can be "broken" at that time and then
/// recreated after.
#[allow(clippy::type_complexity)]
pub fn follow_entity(
    follower_qry: Query<(Entity, &Follow)>,
    mut transform_qry: Query<&mut Transform>,
) {
    for (follower, &Follow(following)) in follower_qry.iter() {
        // Have to borrow mutably to get the change detection logic via Mut<T>
        if let Ok(following_transform) = transform_qry.get_mut(following) {
            if following_transform.is_changed() {
                // Grab the translation so we can drop this mutable borrow and re-run the query
                let translation = following_transform.translation;
                if let Ok(mut transform) = transform_qry.get_mut(follower) {
                    transform.translation = translation;
                }
            }
        }
    }
}

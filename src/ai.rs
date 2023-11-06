use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct DroneAI;

pub fn drone_idle(mut drone_qry: Query<&mut Transform, With<DroneAI>>, time: Res<Time>) {
    for mut drone_transform in drone_qry.iter_mut() {
        let scale = (time.elapsed_seconds() * 4.0).sin() / 20.0 + 1.0;
        drone_transform.scale = Vec3::new(scale, scale, 1.0);

        // FIXME: Moving the drone these sub-pixel amounts causes everything in its fov to blur
        // drone_transform.translation.x += (time.elapsed_seconds() * 4.0 + 5.0).sin() / 15.0;
        // drone_transform.translation.y += (time.elapsed_seconds() * 4.0).sin() / 15.0;

        drone_transform.rotate_z((time.elapsed_seconds() * 4.5 - 6.0).sin() / 600.0);
    }
}

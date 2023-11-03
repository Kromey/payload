use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod fov_update;

#[derive(Debug, Clone, Component)]
pub struct FieldOfView {
    pub view_distance: f32,
    pub view_angle: f32,
    pub mesh: Handle<Mesh>,
    pub texture: Handle<Image>,
}

pub fn update_fov(
    rapier_context: Res<RapierContext>,
    viewer_qry: Query<(Entity, &GlobalTransform, &FieldOfView)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (viewer, viewer_transform, viewer_fov) in viewer_qry.iter() {
        let viewer_pos = viewer_transform.translation().truncate();

        let points = fov_update::cast_view_cone(
            viewer,
            viewer_pos,
            viewer_fov,
            viewer_transform.right().truncate(),
            &rapier_context,
        );

        // Update our mesh
        let mesh = meshes.get_mut(&viewer_fov.mesh).unwrap();
        fov_update::update_view_mesh(mesh, &points, viewer_fov, viewer_pos);
    }
}

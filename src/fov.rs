use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use bevy_rapier2d::prelude::*;

mod helpers;

#[derive(Debug, Clone, Component)]
pub struct FieldOfView {
    pub view_distance: f32,
    pub view_angle: f32,
    pub mesh: Handle<Mesh>,
}

impl FieldOfView {
    pub fn new(view_distance: f32, view_angle: f32) -> Self {
        Self {
            view_distance,
            view_angle,
            mesh: Handle::default(),
        }
    }
}

pub fn add_fov(
    mut fov_query: Query<(Entity, &mut FieldOfView), Added<FieldOfView>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut fov) in fov_query.iter_mut() {
        let image = helpers::make_fov_texture(&fov);

        // Create a mesh to render the field of view to
        let mesh = meshes.add(Mesh::new(PrimitiveTopology::TriangleList));
        // Spawn the layers for rendering field of view
        helpers::make_fov_layers(
            entity,
            image,
            mesh.clone(),
            &mut images,
            &mut commands,
            &mut materials,
        );

        fov.mesh = mesh;
    }
}

pub fn update_fov(
    rapier_context: Res<RapierContext>,
    viewer_qry: Query<(Entity, &GlobalTransform, &FieldOfView)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (viewer, viewer_transform, viewer_fov) in viewer_qry.iter() {
        if let Some(mesh) = meshes.get_mut(&viewer_fov.mesh) {
            let viewer_pos = viewer_transform.translation().truncate();

            let points = helpers::cast_view_cone(
                viewer,
                viewer_pos,
                viewer_fov,
                viewer_transform.right().truncate(),
                &rapier_context,
            );

            // Update our mesh
            helpers::update_view_mesh(mesh, &points, viewer_fov, viewer_pos);
        }
    }
}

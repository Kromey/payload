use bevy::{prelude::*, render::view::RenderLayers};

/// The `Viewable` component controls how entities interact with field of view and line of sight
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Viewable {
    /// Static viewables remain visible once seen, even outside of line of sight, e.g. map tiles, furniture
    Static,
    /// Dynamic viewables are only visible while within line of sight, e.g. raging fungus monsters
    Dynamic,
}

impl From<Viewable> for RenderLayers {
    fn from(value: Viewable) -> Self {
        match value {
            Viewable::Static => RenderLayers::layer(1),
            Viewable::Dynamic => RenderLayers::layer(2),
        }
    }
}

pub fn update_viewables(
    mut commands: Commands,
    viewable_qry: Query<(Entity, &Viewable), Changed<Viewable>>,
) {
    for (entity, &viewable) in viewable_qry.iter() {
        commands.entity(entity).insert(RenderLayers::from(viewable));
    }
}

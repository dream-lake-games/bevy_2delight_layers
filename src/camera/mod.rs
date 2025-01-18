use bevy::prelude::*;

use crate::{
    layer::{layer_defns::SmushLayer, resize_layers_as_needed, Layer, LayerInternal},
    plugin::LayersRes,
    LayersCameraSet,
};

/// This is the component that marks the actual camera location in the world.
/// Invariants:
/// - Either 0 or 1 of these must exist at all time
/// - It must have a Pos
/// The actual camera pos is the IPos of this entity. That means it's only updated during PhysicsSet. Be warned.
#[derive(Component)]
#[require(bevy_2delight_physics::prelude::Pos)]
pub struct DynamicCamera;

/// This should be put on a camera for a layer that uses world space
#[derive(Component)]
pub(crate) struct FollowDynamicCamera;

pub(crate) fn setup_smush_camera(mut commands: Commands, layers_res: Res<LayersRes>) {
    commands
        .spawn((
            Name::new("camera"),
            Camera2d,
            Camera {
                order: SmushLayer::RENDER_ORDER as isize + 1,
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            SmushLayer::RENDER_LAYERS,
        ))
        .set_parent(layers_res.root_eid());
}

fn follow_dynamic_camera(
    dynamic_camera: Query<&bevy_2delight_physics::prelude::Pos, With<DynamicCamera>>,
    mut followers: Query<&mut Transform, (With<FollowDynamicCamera>, Without<DynamicCamera>)>,
    // camera_shake: Res<CameraShake>,
) {
    let Ok(leader) = dynamic_camera.get_single() else {
        return;
    };
    for mut tran in &mut followers {
        tran.translation.x = leader.x.round();
        tran.translation.y = leader.y.round();
    }
}

pub(crate) struct LayersCameraPlugin;
impl Plugin for LayersCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (follow_dynamic_camera, resize_layers_as_needed)
                .after(bevy_2delight_physics::PhysicsSet)
                .in_set(LayersCameraSet),
        );
    }
}

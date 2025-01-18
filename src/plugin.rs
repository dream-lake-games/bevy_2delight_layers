use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};

use crate::{
    camera::{setup_smush_camera, LayersCameraPlugin},
    layer::setup_all_layers,
    light::LayersLightPlugin,
};

#[derive(Resource, Clone)]
pub(crate) struct LayersRes {
    /// How big is the world screen
    pub screen_size: UVec2,
    /// How many multiples of the screen size are things like menu, overlay, transition...
    pub overlay_growth: u32,
    /// Root component
    _root_eid: Entity,
}
impl LayersRes {
    pub(crate) fn root_eid(&self) -> Entity {
        self._root_eid
    }
}

pub(crate) fn init_root_eid(mut commands: Commands, mut layers_res: ResMut<LayersRes>) {
    let root_eid = commands
        .spawn((
            Name::new("LayersRoot"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    layers_res._root_eid = root_eid;
}

pub struct LayersPlugin {
    /// How big is the world screen
    pub screen_size: UVec2,
    /// How many multiples of the screen size are things like menu, overlay, transition...
    pub overlay_growth: u32,
    pub window: Window,
}
impl Plugin for LayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(self.window.clone()),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        app.add_plugins(LayersLightPlugin);
        app.add_plugins(LayersCameraPlugin);

        app.insert_resource(LayersRes {
            screen_size: self.screen_size,
            overlay_growth: self.overlay_growth,
            _root_eid: Entity::PLACEHOLDER,
        });

        app.add_systems(
            Startup,
            (init_root_eid, setup_all_layers, setup_smush_camera).chain(),
        );
    }
}

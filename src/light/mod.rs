use bevy::{asset::embedded_asset, prelude::*, sprite::Material2dPlugin};
use light_alloc::LightAllocer;
use light_cutout_mat::LightCutoutMat;

mod light_alloc;
mod light_cutout_mat;
pub(crate) mod light_man;

pub(crate) struct LayersLightPlugin;
impl Plugin for LayersLightPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "light_cutout_mat.wgsl");

        app.add_plugins(Material2dPlugin::<LightCutoutMat>::default());

        app.insert_resource(LightAllocer::default());
    }
}

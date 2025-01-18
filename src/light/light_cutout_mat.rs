use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

/// After we draw the lights to a render layer and then cut out shapes with black meshes,
/// the image we are left with does not have proper opacity.
/// This shader accomplishes two things:
/// 1. Makes it so that all lights have rgba(1.0, 1.0, 1.0, a) so they blend correctly
/// 2. Makes it so that black maps to clear
/// The point is that we have many lights, and we can just throw the output of all of their "cutouts"
/// into the same layer, they'll blend correctly into a final image, and that is our lighting
/// source of truth.
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct LightCutoutMat {
    #[texture(1)]
    #[sampler(2)]
    light: Handle<Image>,
}
impl Material2d for LightCutoutMat {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_2delight_layers/light/light_cutout_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
impl LightCutoutMat {
    pub fn new(light: Handle<Image>) -> Self {
        Self { light }
    }
}

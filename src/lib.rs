use bevy::prelude::*;

pub mod prelude {
    pub use super::camera::camera_shake::CameraShake;
    pub use super::camera::DynamicCamera;
    pub use super::layer::{
        layer_defns::{
            BgLayer, FgLayer, LightLayer, MainAmbienceLayer, MainDetailLayer, MainStaticLayer,
            MenuLayer, OverlayLayer, TransitionLayer,
        },
        Layer,
    };
    pub use super::light::light_man::{LightAnim, LightDefnPlugin, LightMan};
    pub use super::parallax::{ParallaxX, ParallaxY};
    pub use super::plugin::LayersPlugin;
    pub use super::{LayersCameraSet, LightAnimSet, LightInteractionSet};
}

mod camera;
mod consts;
mod layer;
mod light;
mod parallax;
mod plugin;
mod utils;

/// This is a render layer that explicitly doesn't render
pub const DUMMY_LAYER_USIZE: usize = 0;

/// The set that handles driving underlying light animations. Happens during `PostUpdat`, before `AnimSet`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LightAnimSet;

/// The set that handles light interaction. Happens during `Update`, after `PhysicsSet`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LightInteractionSet;

/// The set that internally handles updating layer cameras. This happens in `PostUpdate`.
/// NOTE: This is the system that places all the cameras. You must make sure the pos is correct before this system.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LayersCameraSet;

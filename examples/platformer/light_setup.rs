use bevy::prelude::*;
use bevy_2delight_anims::prelude::*;
use bevy_2delight_layers::prelude::*;

derive_anim!(
    pub enum Light64Anim {
        #[default]
        #[file("platformer/light_on.png")]
        #[size(64, 64)]
        On,
        #[file("platformer/light_off.png")]
        #[size(64, 64)]
        Off,
    }
);
impl LightAnim for Light64Anim {
    fn light_radius(&self) -> Option<f32> {
        match self {
            Self::On => Some(32.0),
            Self::Off => None,
        }
    }
}
pub(super) type Light64Plugin = LightDefnPlugin<Light64Anim>;

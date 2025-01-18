use bevy::prelude::*;
use bevy_2delight_physics::prelude::Pos;

use crate::{camera::DynamicCamera, LayersCameraSet};

#[derive(Component)]
#[require(Pos)]
pub struct ParallaxX {
    mult: f32,
    wrap_size: Option<f32>,
}
impl ParallaxX {
    pub fn new_wrapped(mult: f32, wrap: f32) -> Self {
        Self {
            mult,
            wrap_size: Some(wrap),
        }
    }
    pub fn new_unwrapped(mult: f32) -> Self {
        Self {
            mult,
            wrap_size: None,
        }
    }
}

#[derive(Component)]
#[require(Pos)]
pub struct ParallaxY {
    mult: f32,
    wrap_size: Option<f32>,
}
impl ParallaxY {
    pub fn new_wrapped(mult: f32, wrap: f32) -> Self {
        Self {
            mult,
            wrap_size: Some(wrap),
        }
    }
    pub fn new_unwrapped(mult: f32) -> Self {
        Self {
            mult,
            wrap_size: None,
        }
    }
}

fn reposition_parallax_x(
    mut px_q: Query<(&Pos, &ParallaxX, &mut Transform)>,
    cam_q: Query<&Pos, With<DynamicCamera>>,
) {
    let Ok(cam_pos) = cam_q.get_single() else {
        return;
    };
    for (px_pos, px_def, mut tran) in &mut px_q {
        let mut diff = (px_pos.x - cam_pos.x) * px_def.mult;
        if let Some(wrap_size) = px_def.wrap_size {
            diff += wrap_size / 2.0;
            diff = diff.rem_euclid(wrap_size);
            diff -= wrap_size / 2.0;
        }

        tran.translation.x = px_pos.x + diff;
    }
}

fn reposition_parallax_y(
    mut py_q: Query<(&Pos, &ParallaxY, &mut Transform)>,
    cam_q: Query<&Pos, With<DynamicCamera>>,
) {
    let Ok(cam_pos) = cam_q.get_single() else {
        return;
    };
    for (py_pos, py_def, mut tran) in &mut py_q {
        let mut diff = (py_pos.y - cam_pos.y) * py_def.mult;
        if let Some(wrap_size) = py_def.wrap_size {
            diff += wrap_size / 2.0;
            diff = diff.rem_euclid(wrap_size);
            diff -= wrap_size / 2.0;
        }

        tran.translation.y = py_pos.y + diff;
    }
}

pub(crate) struct LayersParallaxPlugin;
impl Plugin for LayersParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (reposition_parallax_x, reposition_parallax_y).after(LayersCameraSet),
        );
    }
}

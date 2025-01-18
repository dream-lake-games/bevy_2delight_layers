use bevy::prelude::*;

use super::{Layer, LayerInternal, LayerOutputMode, LayerPositionMode};

const LIGHT_KEY: u32 = 1;
const BG_KEY: u32 = 2;
const MAIN_AMBIENCE_KEY: u32 = 3;
const MAIN_DETAIL_KEY: u32 = 4;
const MAIN_STATIC_LAYER: u32 = 5;
const FG_KEY: u32 = 6;
const OVERLAY_KEY: u32 = 7;
const MENU_KEY: u32 = 8;
const TRANSITION_KEY: u32 = 9;
const SMUSH_KEY: u32 = 10;

/// This is when all the light sources must render + cutout
pub(crate) const PRE_LIGHT_RENDER_ORDER: u32 = 1;
/// This is when light renders
pub(crate) const LIGHT_RENDER_ORDER: u32 = 2;
/// This is everything that depends on light
pub(crate) const POST_LIGHT_RENDER_ORDER: u32 = 3;
/// This is stuff that is smushing together multiple layers, happens last
pub(crate) const SMUSH_RENDER_ORDER: u32 = 4;

#[derive(Debug, Default)]
pub struct LightLayer;
impl Layer for LightLayer {
    const _KEY: u32 = LIGHT_KEY;
}
impl LayerInternal for LightLayer {
    const RENDER_ORDER: u32 = LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = false;
    const ZIX: u32 = 0;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Follow;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::None;
}

#[derive(Debug, Default)]
pub struct BgLayer;
impl Layer for BgLayer {
    const _KEY: u32 = BG_KEY;
}
impl LayerInternal for BgLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = false;
    const ZIX: u32 = LightLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Fixed;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Unlit {
        output_rl: SmushLayer::RENDER_LAYERS,
    };
    const CLEAR_COLOR: Color = Color::BLACK;
}

#[derive(Debug, Default)]
pub struct MainAmbienceLayer;
impl Layer for MainAmbienceLayer {
    const _KEY: u32 = MAIN_AMBIENCE_KEY;
}
impl LayerInternal for MainAmbienceLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = false;
    const ZIX: u32 = BgLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Follow;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Lit {
        base_color: Color::linear_rgb(0.5, 0.5, 0.2),
        output_rl: SmushLayer::RENDER_LAYERS,
    };
}

#[derive(Debug, Default)]
pub struct MainDetailLayer;
impl Layer for MainDetailLayer {
    const _KEY: u32 = MAIN_DETAIL_KEY;
}
impl LayerInternal for MainDetailLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = false;
    const ZIX: u32 = MainAmbienceLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Follow;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Lit {
        base_color: Color::linear_rgb(0.2, 0.2, 0.2),
        output_rl: SmushLayer::RENDER_LAYERS,
    };
}

#[derive(Debug, Default)]
pub struct MainStaticLayer;
impl Layer for MainStaticLayer {
    const _KEY: u32 = MAIN_STATIC_LAYER;
}
impl LayerInternal for MainStaticLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = false;
    const ZIX: u32 = MainDetailLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Follow;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Unlit {
        output_rl: SmushLayer::RENDER_LAYERS,
    };
}

#[derive(Debug, Default)]
pub struct FgLayer;
impl Layer for FgLayer {
    const _KEY: u32 = FG_KEY;
}
impl LayerInternal for FgLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = false;
    const ZIX: u32 = MainStaticLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Fixed;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Unlit {
        output_rl: SmushLayer::RENDER_LAYERS,
    };
}

#[derive(Debug, Default)]
pub struct OverlayLayer;
impl Layer for OverlayLayer {
    const _KEY: u32 = OVERLAY_KEY;
}
impl LayerInternal for OverlayLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = true;
    const ZIX: u32 = FgLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Fixed;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Unlit {
        output_rl: SmushLayer::RENDER_LAYERS,
    };
}

#[derive(Debug, Default)]
pub struct MenuLayer;
impl Layer for MenuLayer {
    const _KEY: u32 = MENU_KEY;
}
impl LayerInternal for MenuLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = true;
    const ZIX: u32 = OverlayLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Fixed;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Unlit {
        output_rl: SmushLayer::RENDER_LAYERS,
    };
}

#[derive(Debug, Default)]
pub struct TransitionLayer;
impl Layer for TransitionLayer {
    const _KEY: u32 = TRANSITION_KEY;
}
impl LayerInternal for TransitionLayer {
    const RENDER_ORDER: u32 = POST_LIGHT_RENDER_ORDER;
    const IS_OVERLAY: bool = true;
    const ZIX: u32 = MenuLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Fixed;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::Unlit {
        output_rl: SmushLayer::RENDER_LAYERS,
    };
}

#[derive(Debug, Default)]
pub struct SmushLayer;
impl Layer for SmushLayer {
    const _KEY: u32 = SMUSH_KEY;
}
impl LayerInternal for SmushLayer {
    const RENDER_ORDER: u32 = SMUSH_RENDER_ORDER;
    const IS_OVERLAY: bool = true;
    const ZIX: u32 = TransitionLayer::ZIX + 1;
    const LAYER_POSITION_MODE: LayerPositionMode = LayerPositionMode::Fixed;
    const LAYER_OUTPUT_MODE: LayerOutputMode = LayerOutputMode::None;
    const CLEAR_COLOR: Color = Color::WHITE;
}

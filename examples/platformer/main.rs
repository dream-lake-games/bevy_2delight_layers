use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::view::RenderLayers,
    window::{WindowMode, WindowResolution},
};
use bevy_2delight_layers::prelude::*;
use bevy_2delight_physics::prelude::*;
use physics_setup::{
    physics_update, PhysicsPlugin, TriggerRx, TriggerRxKind, TriggerTx, TriggerTxKind,
};

mod physics_setup;

const SCREEN_UVEC: UVec2 = UVec2::new(240, 240);
const SCREEN_VEC: Vec2 = Vec2::new(SCREEN_UVEC.x as f32, SCREEN_UVEC.y as f32);
const OVERLAY_GROWTH: u32 = 3;
const OVERLAY_UVEC: UVec2 = UVec2::new(
    SCREEN_UVEC.x * OVERLAY_GROWTH,
    SCREEN_UVEC.y * OVERLAY_GROWTH,
);
const OVERLAY_VEC: Vec2 = Vec2::new(OVERLAY_UVEC.x as f32, OVERLAY_UVEC.y as f32);

fn main() {
    let mut app = App::new();

    app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
    app.add_plugins(PhysicsPlugin::default());
    app.add_plugins(LayersPlugin {
        screen_size: SCREEN_UVEC,
        overlay_growth: OVERLAY_GROWTH,
        window: Window {
            resizable: true,
            title: "bevy_2delight_layers".to_string(),
            resolution: WindowResolution::new(OVERLAY_VEC.x, OVERLAY_VEC.y),
            mode: WindowMode::Windowed,
            ..default()
        },
    });

    app.add_systems(Startup, startup);
    app.add_systems(Update, physics_update.after(PhysicsSet));

    app.run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
#[require(Name(|| Name::new("Ground")))]
struct Ground;
#[derive(Bundle)]
struct GroundBundle {
    ground: Ground,
    pos: Pos,
    sprite: Sprite,
    static_tx: StaticTx,
    rl: RenderLayers,
}
impl GroundBundle {
    fn new(pos: Pos, size: UVec2) -> Self {
        Self {
            ground: Ground,
            pos,
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                ..default()
            },
            static_tx: StaticTx::single(StaticTxKind::Solid, HBox::new(size.x, size.y)),
            rl: MainStaticLayer::RENDER_LAYERS,
        }
    }
}

#[derive(Component)]
#[require(Name(|| Name::new("Spike")))]
struct Spike;
#[derive(Bundle)]
struct SpikeBundle {
    spike: Spike,
    pos: Pos,
    sprite: Sprite,
    trigger_tx: TriggerTx,
    rl: RenderLayers,
}
impl SpikeBundle {
    fn new(pos: Pos, size: UVec2) -> Self {
        Self {
            spike: Spike,
            pos,
            sprite: Sprite {
                custom_size: Some(size.as_vec2()),
                color: Color::linear_rgb(1.0, 0.0, 0.0),
                ..default()
            },
            trigger_tx: TriggerTx::single(TriggerTxKind::Spikes, HBox::new(size.x, size.y)),
            rl: MainStaticLayer::RENDER_LAYERS,
        }
    }
}

fn startup(mut commands: Commands) {
    let player_hbox = HBox::new(12, 12);
    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite {
            custom_size: Some(player_hbox.get_size().as_vec2()),
            color: Color::linear_rgb(0.1, 1.0, 0.1),
            ..default()
        },
        Pos::new(0.0, 0.0),
        Dyno::new(0.0, 0.0),
        StaticRx::single(StaticRxKind::Default, player_hbox.clone()),
        TriggerRx::single(TriggerRxKind::Player, player_hbox.clone()),
        MainStaticLayer::RENDER_LAYERS,
    ));

    commands.spawn(GroundBundle::new(
        Pos::new(0.0, -SCREEN_VEC.y / 2.0),
        UVec2::new(SCREEN_UVEC.x, 24),
    ));
    commands.spawn(GroundBundle::new(
        Pos::new(-SCREEN_VEC.x / 2.0, 0.0),
        UVec2::new(SCREEN_UVEC.x / 2, 12),
    ));
    commands.spawn(GroundBundle::new(
        Pos::new(SCREEN_VEC.x / 2.0, 0.0),
        UVec2::new(SCREEN_UVEC.x / 2, 12),
    ));

    commands.spawn(SpikeBundle::new(
        Pos::new(-SCREEN_VEC.x / 2.0, 18.0),
        UVec2::new(36, 24),
    ));
    commands.spawn(SpikeBundle::new(
        Pos::new(SCREEN_VEC.x / 2.0, 18.0),
        UVec2::new(36, 24),
    ));
}

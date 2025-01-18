use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::view::RenderLayers,
    window::{WindowMode, WindowResolution},
};
use bevy_2delight_anims::prelude::AnimPlugin;
use bevy_2delight_layers::prelude::*;
use bevy_2delight_physics::prelude::*;
use light_setup::{Light64Anim, Light64Plugin};
use physics_setup::{
    physics_update, PhysicsPlugin, TriggerRx, TriggerRxKind, TriggerTx, TriggerTxKind,
};

mod light_setup;
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
    app.add_plugins(AnimPlugin::new());
    app.add_plugins(Light64Plugin::default());

    app.add_systems(Startup, startup);
    app.add_systems(
        Update,
        (
            physics_update,
            camera_follow_player,
            toggle_light,
            shake_big_collisions,
        )
            .after(PhysicsSet)
            .before(LayersCameraSet),
    );

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
        }
    }
}

fn startup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn(DynamicCamera);

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
        LightMan::new(Light64Anim::On),
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

    commands.spawn((
        SpikeBundle::new(Pos::new(-SCREEN_VEC.x / 2.0, 18.0), UVec2::new(36, 24)),
        MainAmbienceLayer::RENDER_LAYERS,
    ));
    commands.spawn((
        SpikeBundle::new(Pos::new(SCREEN_VEC.x / 2.0, 18.0), UVec2::new(36, 24)),
        MainDetailLayer::RENDER_LAYERS,
    ));

    commands.spawn((
        Name::new("Bg"),
        Sprite {
            image: ass.load("platformer/bg.png"),
            custom_size: Some(SCREEN_VEC),
            ..default()
        },
        BgLayer::RENDER_LAYERS,
        ParallaxX::new_wrapped(0.3, SCREEN_VEC.x),
        ParallaxY::new_wrapped(0.3, SCREEN_VEC.y),
    ));
    commands.spawn((
        Name::new("Fg"),
        Sprite {
            image: ass.load("platformer/fg.png"),
            custom_size: Some(Vec2::new(SCREEN_VEC.x * 3.0, SCREEN_VEC.y)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            ..default()
        },
        FgLayer::RENDER_LAYERS,
        ParallaxX::new_wrapped(1.2, SCREEN_VEC.x * 2.0),
    ));

    commands.spawn((
        Name::new("MoreAmbience"),
        Transform::from_translation(Vec3::Z * -5.0),
        Sprite {
            color: Color::linear_rgb(0.4, 0.4, 0.5),
            custom_size: Some(Vec2::new(120.0, 360.0)),
            ..default()
        },
        MainAmbienceLayer::RENDER_LAYERS,
    ));

    commands.spawn((
        Name::new("OverlayText"),
        Text2d::new("Overlay Text"),
        Transform::from_translation(Vec3::new(0.0, SCREEN_VEC.y / 2.0 - 12.0, 0.0)),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        OverlayLayer::RENDER_LAYERS,
    ));
    commands.spawn((
        Name::new("MenuText"),
        Text2d::new("Menu Text"),
        Transform::from_translation(Vec3::new(0.0, SCREEN_VEC.y / 2.0 - 24.0, 0.0)),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        MenuLayer::RENDER_LAYERS,
    ));
    commands.spawn((
        Name::new("TransitionText"),
        Text2d::new("Transition Text"),
        Transform::from_translation(Vec3::new(0.0, SCREEN_VEC.y / 2.0 - 36.0, 0.0)),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TransitionLayer::RENDER_LAYERS,
    ));
}

fn camera_follow_player(
    mut camera_q: Query<&mut Pos, (With<DynamicCamera>, Without<Player>)>,
    player_q: Query<&Pos, (Without<DynamicCamera>, With<Player>)>,
) {
    let mut cam_pos = camera_q.single_mut();
    let player_pos = player_q.single();
    *cam_pos = player_pos.clone();
}

fn toggle_light(
    mut player_q: Query<&mut LightMan<Light64Anim>, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut light_man = player_q.single_mut();
    if keyboard.just_pressed(KeyCode::KeyQ) {
        light_man.set_state(Light64Anim::Off);
    }
    if keyboard.just_pressed(KeyCode::KeyE) {
        light_man.set_state(Light64Anim::On);
    }
}

fn shake_big_collisions(
    static_colls: Res<StaticColls>,
    player_q: Query<&StaticRx, With<Player>>,
    mut shake: ResMut<CameraShake>,
) {
    let player_srx = player_q.single();
    if static_colls
        .get_refs(&player_srx.coll_keys)
        .iter()
        .any(|coll| coll.rx_perp.length_squared() > 500.0)
    {
        shake.add_shake(0.1, -1..=1, -2..=2);
    }
}

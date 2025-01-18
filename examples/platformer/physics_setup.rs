use bevy::prelude::*;
use bevy_2delight_physics::prelude::*;

use crate::Player;

#[derive(std::hash::Hash, Debug, Clone)]
pub(crate) enum TriggerRxKind {
    Player,
}
impl TriggerKind for TriggerRxKind {}

#[derive(std::hash::Hash, Debug, Clone, PartialEq, Eq)]
pub(crate) enum TriggerTxKind {
    Spikes,
}
impl TriggerKind for TriggerTxKind {}

#[derive(Default, Debug, Clone)]
pub(crate) enum BulletTimeSpeed {
    #[default]
    Normal,
    Slow,
}
impl BulletTimeClass for BulletTimeSpeed {
    fn to_factor(&self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Slow => 0.1,
        }
    }
}
impl BulletTimeSpeed {
    pub fn rotated(&self) -> Self {
        match self {
            Self::Normal => Self::Slow,
            Self::Slow => Self::Normal,
        }
    }
}

// I _highly_ recommend you create type aliases here to cut back on some verbosity
pub(crate) type TriggerRx = TriggerRxGeneric<TriggerRxKind>;
pub(crate) type TriggerTx = TriggerTxGeneric<TriggerTxKind>;
pub(crate) type TriggerColls = TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>;
#[allow(dead_code)]
pub(crate) type TriggerCollRec = TriggerCollRecGeneric<TriggerRxKind, TriggerTxKind>;
pub(crate) type BulletTime = BulletTimeGeneric<BulletTimeSpeed>;
pub(crate) type PhysicsPlugin = PhysicsPluginGeneric<TriggerRxKind, TriggerTxKind, BulletTimeSpeed>;

pub(crate) fn physics_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut bullet_time: ResMut<BulletTime>,
    mut player_q: Query<(&mut Pos, &mut Dyno, &mut Sprite, &StaticRx, &TriggerRx), With<Player>>,
    static_colls: Res<StaticColls>,
    trigger_colls: Res<TriggerColls>,
) {
    // Maybe toggle bullet time
    if keyboard.just_pressed(KeyCode::Space) {
        let new_speed = bullet_time.get_base().rotated();
        bullet_time.set_base(new_speed);
    }

    let (mut pos, mut dyno, mut sprite, srx, trx) = player_q.single_mut();

    // Horizontal movement
    let x_mag = 200.0;
    dyno.vel.x = 0.0;
    if keyboard.pressed(KeyCode::KeyA) {
        dyno.vel.x -= x_mag;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dyno.vel.x += x_mag;
    }

    // Vertical movement
    let gravity_mag = 600.0;
    let jump_mag = 400.0;
    dyno.vel.y -= bullet_time.delta_secs() * gravity_mag;
    if keyboard.just_pressed(KeyCode::KeyW) {
        dyno.vel.y = jump_mag;
        // Commenting out because it feels bad but here's how to add a short-term bullet time effect
        // bullet_time.add_effect(BulletTimeSpeeds::Slow, 0.1);
    }

    // How to check for collisions
    if static_colls
        .get_refs(&srx.coll_keys)
        .iter()
        .any(|coll| coll.tx_kind == StaticTxKind::Solid)
    {
        sprite.color = Color::linear_rgb(0.1, 1.0, 1.0);
    } else {
        sprite.color = Color::linear_rgb(0.1, 1.0, 0.1);
    }
    if trigger_colls
        .get_refs(&trx.coll_keys)
        .iter()
        .any(|coll| coll.tx_kind == TriggerTxKind::Spikes)
    {
        *pos = Pos::default();
    }
}

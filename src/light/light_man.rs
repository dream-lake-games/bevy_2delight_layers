use bevy::prelude::*;
use bevy_2delight_anims::AnimSet;

use crate::LightAnimSet;

use super::light_alloc::LightClaim;

/// A trait that will allow lighting systems to use this anim as light source
pub trait LightAnim: bevy_2delight_anims::prelude::AnimStateMachine {
    /// How far does this light extend? Helps prune light interaction calcs
    fn radius(&self) -> Option<f32>;
}

/// Records what kind of state update needs to happen internally
pub(crate) enum LightStateUpdate<Anim: LightAnim> {
    Set(Anim),
    Reset(Anim),
}
impl<Anim: LightAnim> LightStateUpdate<Anim> {
    pub(crate) fn get_state(&self) -> &Anim {
        match self {
            Self::Set(state) => state,
            Self::Reset(state) => state,
        }
    }
}

/// A light source manager, implemented as a thin wrapper around AnimMan
/// TODO: Figure out if this makes sense, get_state seems fucky... idk... also implement flips
#[derive(Component)]
#[component(on_add = on_add_light_man::<Anim>)]
#[component(on_remove = on_remove_light_man::<Anim>)]
pub struct LightMan<Anim: LightAnim> {
    pub(crate) state_update: Option<LightStateUpdate<Anim>>,
    pub(super) claim: LightClaim,
}
/// Responsible for getting a light claim from the world and creating the underlying anim
fn on_add_light_man<Anim: LightAnim>(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    // Get da claim
    let claim = LightClaim::alloc(&mut world);
    let mut myself = world.get_mut::<LightMan<Anim>>(eid).unwrap();
    myself.claim = claim;
    let start_state = myself
        .state_update
        .as_mut()
        .map(|inner| inner.get_state())
        .cloned();

    // Make da anim
    world
        .commands()
        .entity(eid)
        .insert(bevy_2delight_anims::prelude::AnimMan::new(
            start_state.unwrap_or_default(),
        ));
}
/// Responsible for releasing the light claim and removing the underlying anim
fn on_remove_light_man<Anim: LightAnim>(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let claim = world.get::<LightMan<Anim>>(eid).unwrap().claim.clone();
    claim.free(&mut world);
    world
        .commands()
        .entity(eid)
        .remove::<bevy_2delight_anims::prelude::AnimMan<Anim>>();
}
impl<Anim: LightAnim> LightMan<Anim> {
    pub fn new(state: Anim) -> Self {
        Self {
            state_update: Some(LightStateUpdate::Reset(state)),
            claim: default(),
        }
    }
    pub fn with_state(mut self, state: Anim) -> Self {
        self.state_update = Some(LightStateUpdate::Set(state));
        self
    }
    pub fn set_state(&mut self, state: Anim) {
        self.state_update = Some(LightStateUpdate::Set(state));
    }
    pub fn reset_state(&mut self, state: Anim) {
        self.state_update = Some(LightStateUpdate::Reset(state));
    }
}

fn drive_light_anims<Anim: LightAnim>(
    mut light_q: Query<(
        Entity,
        &mut LightMan<Anim>,
        Option<&mut bevy_2delight_anims::prelude::AnimMan<Anim>>,
    )>,
    mut commands: Commands,
) {
    for (eid, mut light, mut anim) in &mut light_q {
        match anim.as_mut() {
            Some(anim) => match light.state_update {
                Some(LightStateUpdate::Set(state)) => {
                    anim.set_state(state);
                    light.state_update = None;
                }
                Some(LightStateUpdate::Reset(state)) => {
                    anim.reset_state(state);
                    light.state_update = None;
                }
                None => {}
            },
            None => {
                // If the underlying anim is removed, we interpret that as meaning we should remove the light as well
                commands.entity(eid).remove::<LightMan<Anim>>();
            }
        }
    }
}

#[derive(Default)]
pub struct LightManPlugin<Anim: LightAnim> {
    _pd: std::marker::PhantomData<Anim>,
}
impl<Anim: LightAnim> Plugin for LightManPlugin<Anim> {
    fn build(&self, app: &mut App) {
        // TODO: Light interaction per source
        app.add_systems(
            PostUpdate,
            drive_light_anims::<Anim>
                .after(bevy_2delight_anims::AnimSet)
                .in_set(LightAnimSet)
                .before(AnimSet),
        );
    }
}

use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const STAR_SIZE: f32 = 40.0;
pub const NUM_OF_STARS: usize = 10;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}

use crate::resources::world_bounds::WorldBounds;
use amethyst::{
    core::{
        math::{Vector2, Vector3},
        timing::Time,
        transform::components::Transform,
    },
    ecs::*,
    shrev::EventChannel,
};

use rand::{thread_rng, Rng};
use std::f32;

use crate::{
    components::creatures::Movement, resources::wind::Wind, systems::spawner::CreatureSpawnEvent,
};

/// A new topplegrass entity is spawned periodically, SPAWN_INTERVAL is the period in seconds.
const SPAWN_INTERVAL: f32 = 0.5;
/// The standard scaling to apply to the entity.
const TOPPLEGRASS_BASE_SCALE: f32 = 0.002;
/// The maximum movement speed of Topplegrass.
const MAX_MOVEMENT_SPEED: f32 = 1.75;

/// Periodically spawns a Topplegrass entity.
#[derive(Default)]
pub struct TopplegrassSpawnSystem {
    secs_to_next_spawn: f32,
}

/// Periodically schedules a Topplegrass entity to be spawned in through a CreatureSpawnEvent.
impl<'s> System<'s> for TopplegrassSpawnSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Write<'s, EventChannel<CreatureSpawnEvent>>,
        Read<'s, Time>,
        Read<'s, WorldBounds>,
        Read<'s, Wind>,
    );

    fn run(
        &mut self,
        (entities, lazy_update, mut spawn_events, time, world_bounds, wind): Self::SystemData,
    ) {
        if self.ready_to_spawn(time.delta_seconds()) {
            let mut transform = Transform::default();
            transform.set_scale(Vector3::new(
                TOPPLEGRASS_BASE_SCALE,
                TOPPLEGRASS_BASE_SCALE,
                TOPPLEGRASS_BASE_SCALE,
            ));
            transform.append_translation(Self::gen_spawn_location(&wind, &world_bounds));
            let movement = Movement {
                velocity: Vector3::new(wind.wind.x, wind.wind.y, 0.0),
                max_movement_speed: MAX_MOVEMENT_SPEED,
            };
            let entity = lazy_update
                .create_entity(&entities)
                .with(transform)
                .with(movement)
                .build();
            spawn_events.single_write(CreatureSpawnEvent {
                creature_type: "Topplegrass".to_string(),
                entity,
            });
        }
    }
}

impl TopplegrassSpawnSystem {
    /// Checks the time elapsed since the last spawn. If the system is ready to spawn another
    /// entity, the timer will be reset and this function will return true.
    fn ready_to_spawn(&mut self, delta_seconds: f32) -> bool {
        self.secs_to_next_spawn -= delta_seconds;
        if self.secs_to_next_spawn.is_sign_negative() {
            self.secs_to_next_spawn = SPAWN_INTERVAL;
            true
        } else {
            false
        }
    }

    /// Returns a Vector3<f32> representing the position in which to spawn the next entity.
    /// Entities will be spawned at a random point on one of the four world borders; specifically,
    /// the one that the wind direction is facing away from. In other words: upwind from the
    /// center of the world.
    fn gen_spawn_location(wind: &Wind, bounds: &WorldBounds) -> Vector3<f32> {
        let mut rng = thread_rng();
        if Self::wind_towards_direction(wind.wind, Vector2::new(1.0, 0.0)) {
            Vector3::new(bounds.left, rng.gen_range(bounds.bottom, bounds.top), 0.5)
        } else if Self::wind_towards_direction(wind.wind, Vector2::new(0.0, 1.0)) {
            Vector3::new(rng.gen_range(bounds.left, bounds.right), bounds.bottom, 0.5)
        } else if Self::wind_towards_direction(wind.wind, Vector2::new(-1.0, 0.0)) {
            Vector3::new(bounds.right, rng.gen_range(bounds.bottom, bounds.top), 0.5)
        } else {
            Vector3::new(rng.gen_range(bounds.left, bounds.right), bounds.top, 0.5)
        }
    }

    /// Returns true if and only if the given wind vector is roughly in line with the given
    /// cardinal_direction vector, within a margin of a 1/4 PI RAD.
    fn wind_towards_direction(wind: Vector2<f32>, cardinal_direction: Vector2<f32>) -> bool {
        wind.angle(&cardinal_direction).abs() < f32::consts::FRAC_PI_4
    }
}

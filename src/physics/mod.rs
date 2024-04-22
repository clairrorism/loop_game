use crate::input::PlayerAction;
use crate::{combat::*, Player};
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

pub mod collision;
pub mod movement;

#[derive(Bundle)]
pub struct TerrainBundle {
    transform: Transform,
    collider: StaticCollider,
    terrain: Terrain,
}

impl TerrainBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            collider: StaticCollider(make_aabb(&transform)),
            terrain: Terrain,
        }
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DynamicCollision>()
            .add_event::<StaticCollision>()
            .add_event::<TerrainCollision>()
            .add_systems(
                FixedUpdate,
                (
                    (apply_gravity, do_player_movement),
                    apply_velocity,
                    (
                        check_static_collisions,
                        check_dynamic_collisions,
                        reset_player_velocity,
                    ),
                    terrain_collision_handler,
                )
                    .chain(),
            );
    }
}

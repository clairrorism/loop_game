use self::collision::*;
use self::movement::*;
use crate::input::PlayerAction;
use crate::{combat::*, Player};
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

pub mod prelude {
    pub use super::collision::*;
    pub use super::movement::*;
}

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
            .add_systems(Update, do_player_movement.before(apply_velocity))
            .add_systems(
                FixedUpdate,
                (
                    apply_gravity,
                    apply_velocity,
                    (
                        check_static_collisions,
                        check_dynamic_collisions,
                        // reset_player_velocity,
                    ),
                    terrain_collision_handler,
                    #[cfg(feature = "show_hitbox")]
                    show_phys_things,
                )
                    .chain(),
            );
    }
}

fn show_phys_things(
    mut gizmos: Gizmos,
    query: Query<(&Transform, Option<&Player>), With<Collider>>,
    terrain_query: Query<(&StaticCollider, Option<&Terrain>)>,
) {
    for (xform, maybe_player) in &query {
        gizmos.rect_2d(
            xform.translation.truncate(),
            0.,
            xform.scale.truncate(),
            match maybe_player {
                Some(_) => Color::GREEN,
                None => Color::RED,
            },
        );
    }

    for (StaticCollider(hitbox), maybe_terrain) in &terrain_query {
        gizmos.rect_2d(
            hitbox.center(),
            0.,
            hitbox.half_size() * 2.,
            match maybe_terrain {
                Some(_) => Color::BLUE,
                None => Color::YELLOW,
            },
        );
    }
}

use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

use crate::movement::{GravityAffected, Velocity};

use super::DeathEvent;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut)]
pub struct StaticCollider(pub Aabb2d);

#[derive(Event, Clone, Copy)]
pub struct DynamicCollision(pub Entity, pub Entity);

#[derive(Event, Clone, Copy)]
pub struct StaticCollision {
    pub static_entity: Entity,
    pub other: Entity,
}

#[derive(Event, Clone, Copy)]
pub struct TerrainCollision {
    pub terrain: Entity,
    pub other: Entity,
}

#[derive(Component)]
pub enum TerrainHandler {
    Die,
    Stop,
}

#[derive(Component)]
pub struct Terrain;

pub fn make_aabb(transform: &Transform) -> Aabb2d {
    Aabb2d::new(
        transform.translation.truncate(),
        transform.scale.truncate() / 2.,
    )
}
pub(super) fn check_dynamic_collisions(
    query: Query<(Entity, &Transform), With<Collider>>,
    mut events: EventWriter<DynamicCollision>,
) {
    query.iter_combinations().for_each(|[(e1, t1), (e2, t2)]| {
        if make_aabb(t1).intersects(&make_aabb(t2)) {
            events.send(DynamicCollision(e1, e2));
        }
    });
}
pub(super) fn check_static_collisions(
    actor_query: Query<(Entity, &Transform), With<Collider>>,
    static_query: Query<(Entity, &StaticCollider, Option<&Terrain>)>,
    mut static_events: EventWriter<StaticCollision>,
    mut terrain_events: EventWriter<TerrainCollision>,
) {
    static_query
        .iter()
        .for_each(|(e1, static_collider, maybe_terrain)| {
            for (e2, atr) in actor_query.iter() {
                let hitbox = make_aabb(atr);
                if !hitbox.intersects(&(static_collider.0)) {
                    continue;
                }
                match maybe_terrain {
                    Some(_) => {
                        terrain_events.send(TerrainCollision {
                            terrain: e1,
                            other: e2,
                        });
                    }
                    None => {
                        static_events.send(StaticCollision {
                            static_entity: e1,
                            other: e2,
                        });
                    }
                }
            }
        });
}
pub(super) fn terrain_collision_handler(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        Option<&mut GravityAffected>,
        &TerrainHandler,
    )>,
    terrain_query: Query<&StaticCollider>,
    mut terrain_events: EventReader<TerrainCollision>,
    mut death_events: EventWriter<DeathEvent>,
) {
    for TerrainCollision { terrain, other } in terrain_events.read() {
        let Ok((mut xform, mut vel, maybe_grav, handler)) = query.get_mut(*other) else {
            continue;
        };

        if matches!(handler, TerrainHandler::Die) {
            death_events.send(DeathEvent(*other));
            continue;
        }

        let Ok(terrain_hitbox) = terrain_query.get(*terrain) else {
            error!("Attempted to get nonexisting terrain hitbox.");
            continue;
        };

        let closest = terrain_hitbox.closest_point(xform.translation.truncate());
        let center = terrain_hitbox.center();
        let half_x = terrain_hitbox.half_size().x;

        #[cfg(feature = "log_terrain_collision")]
        let side: &str;

        if center.x - half_x < closest.x && closest.x < center.x + half_x {
            // WITHOUT THIS CHECK CORNERS BECOME SUPER SNAPPY AND TERRIBLE
            if !((closest.x > center.x && (xform.translation.x > closest.x))
                || (closest.x < center.x && (xform.translation.x < closest.x)))
            {
                xform.translation.y = if closest.y > center.y {
                    // top collision
                    if let Some(mut grav) = maybe_grav {
                        grav.is_airborne = false;
                    }

                    /*if cfg!(feature = "log_terrain_collision") {
                        side = "Top";
                    }*/

                    closest.y + (xform.scale.y / 2.)
                } else {
                    // bottom collision
                    closest.y - (xform.scale.y / 2.)
                };
            }
            vel.y = 0.;
        } else {
            // Ditto but for x axis stuff
            if !((closest.y > center.y && (xform.translation.y > closest.y))
                || (closest.y < center.y && (xform.translation.y < closest.y)))
            {
                xform.translation.x = if closest.x > center.x {
                    // right collision
                    closest.x + (xform.scale.x / 2.)
                } else {
                    // left collision
                    closest.x - (xform.scale.x / 2.)
                };
            }
            vel.x = 0.;
        }
    }
}

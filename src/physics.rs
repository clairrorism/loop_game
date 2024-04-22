use crate::input::PlayerAction;
use crate::{combat::*, Player};
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct GravityAffected {
    pub is_airborne: bool,
}

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

fn make_aabb(transform: &Transform) -> Aabb2d {
    Aabb2d::new(
        transform.translation.truncate(),
        transform.scale.truncate() / 2.,
    )
}

fn check_dynamic_collisions(
    query: Query<(Entity, &Transform), With<Collider>>,
    mut events: EventWriter<DynamicCollision>,
) {
    query.iter_combinations().for_each(|[(e1, t1), (e2, t2)]| {
        if make_aabb(t1).intersects(&make_aabb(t2)) {
            events.send(DynamicCollision(e1, e2));
        }
    });
}

fn check_static_collisions(
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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut xform, vel) in &mut query {
        xform.translation.x += vel.x * time.delta_seconds();
        xform.translation.y += vel.y * time.delta_seconds();
    }
}

fn apply_gravity(mut query: Query<&mut Velocity, With<GravityAffected>>, time: Res<Time>) {
    const GRAVITY_MULTIPLIER: f32 = 9.8 * 80.;

    for mut vel in &mut query {
        vel.y -= GRAVITY_MULTIPLIER * time.delta_seconds();
    }
}

fn terrain_collision_handler(
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

        if center.x - half_x < closest.x && closest.x < center.x + half_x {
            xform.translation.y = if closest.y > center.y {
                if let Some(mut grav) = maybe_grav {
                    grav.is_airborne = false;
                }
                closest.y + (xform.scale.y / 2.)
            } else {
                closest.y - (xform.scale.y / 2.)
            };
            vel.y = 0.;
        } else {
            xform.translation.x = if closest.x > center.x {
                info!("Right collision!");
                closest.x + (xform.scale.x / 2.)
            } else {
                info!("Left collision!");
                closest.x - (xform.scale.x / 2.)
            };
            vel.x = 0.;
        }
    }
}

fn do_player_movement(
    mut query: Query<(&mut Transform, &mut Velocity, &mut GravityAffected), With<Player>>,
    mut events: EventReader<PlayerAction>,
) {
    const PLAYER_MS: f32 = 150.;
    const PLAYER_JUMPSPEED: f32 = 375.;
    if let Ok((mut xform, mut vel, mut grav)) = query.get_single_mut() {
        for ev in events.read() {
            match ev {
                PlayerAction::MoveRight => {
                    vel.x = PLAYER_MS;
                }
                PlayerAction::MoveLeft => {
                    vel.x = -PLAYER_MS;
                }
                PlayerAction::Crouch => todo!(),
                PlayerAction::Jump => {
                    if !grav.is_airborne {
                        xform.translation.y += 3.;
                        vel.y = PLAYER_JUMPSPEED;
                        grav.is_airborne = true;
                    }
                }
                PlayerAction::Attack => todo!(),
                PlayerAction::Interact => todo!(),
            }
        }
    }
}

pub fn reset_player_velocity(mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut vel) = query.get_single_mut() {
        vel.x = 0.;
    }
}

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
                    (check_static_collisions, check_dynamic_collisions),
                    terrain_collision_handler,
                )
                    .chain(),
            );
    }
}

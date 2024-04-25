use bevy::prelude::*;

use crate::{input::PlayerAction, Player};

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct GravityAffected {
    pub is_airborne: bool,
}

#[derive(Component, PartialEq)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Component)]
pub struct FollowsPlayer;

pub(super) fn do_player_movement(
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut GravityAffected,
            &mut Facing,
        ),
        With<Player>,
    >,
    keys: Res<ButtonInput<KeyCode>>,
) {
    const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
    const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
    const JUMP: [KeyCode; 3] = [KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp];
    const CROUCH: [KeyCode; 3] = [KeyCode::ShiftLeft, KeyCode::KeyS, KeyCode::ArrowDown];

    const PLAYER_MS: f32 = 150.;
    const PLAYER_JUMPSPEED: f32 = 375.;

    if let Ok((mut xform, mut vel, mut grav, mut orientation)) = query.get_single_mut() {
        let right = keys.any_pressed(RIGHT);
        let left = keys.any_pressed(LEFT);

        vel.x = if right ^ left {
            if right {
                orientation.set_if_neq(Facing::Right);
                PLAYER_MS
            } else {
                orientation.set_if_neq(Facing::Left);
                -PLAYER_MS
            }
        } else {
            0.
        };

        if !grav.is_airborne && keys.any_just_pressed(JUMP) {
            xform.translation.y += 3.;
            vel.y = PLAYER_JUMPSPEED;
            grav.is_airborne = true;
        }

        if vel.y < -10. {
            grav.is_airborne = true
        }
    }
}

pub(super) fn apply_velocity(
    mut query: Query<(&mut Transform, Option<&mut Facing>, &Velocity)>,
    time: Res<Time>,
) {
    for (mut xform, maybe_facing, vel) in &mut query {
        xform.translation.x += vel.x * time.delta_seconds();
        xform.translation.y += vel.y * time.delta_seconds();
    }
}

pub(super) fn apply_gravity(
    mut query: Query<&mut Velocity, With<GravityAffected>>,
    time: Res<Time>,
) {
    const GRAVITY_MULTIPLIER: f32 = 9.8 * 80.;

    for mut vel in &mut query {
        vel.y -= GRAVITY_MULTIPLIER * time.delta_seconds();
    }
}

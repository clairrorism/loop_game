use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct GravityAffected {
    pub is_airborne: bool,
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

pub fn reset_player_velocity(mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut vel) = query.get_single_mut() {
        vel.x = 0.;
    }
}

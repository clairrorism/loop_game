use bevy::math::bounding::{Aabb2d, BoundingVolume};
use bevy::prelude::*;

pub mod combat;
pub mod input;
pub mod physics;

use physics::*;

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugin))
        .add_event::<combat::DeathEvent>()
        .add_event::<input::PlayerAction>()
        .add_systems(Update, show_phys_things)
        .add_systems(Update, input::handle_input)
        .add_systems(FixedUpdate, physics::reset_player_velocity)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(TerrainBundle::new(Transform {
        translation: Vec3::new(0., -100., 0.),
        scale: Vec3::new(300., 30., 1.),
        ..default()
    }));

    commands.spawn(TerrainBundle::new(Transform {
        translation: Vec3::new(125., -70., 0.),
        scale: Vec3::new(50., 100., 0.),
        ..default()
    }));

    commands.spawn((
        Transform {
            translation: Vec3::new(0., 100., 0.),
            scale: Vec3::new(20., 20., 1.),
            ..default()
        },
        Velocity(Vec2::new(0., 0.)),
        TerrainHandler::Stop,
        GravityAffected { is_airborne: true },
        Collider,
        Player,
    ));
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

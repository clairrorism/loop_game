use bevy::prelude::*;

#[derive(Event)]
pub struct DeathEvent(pub Entity);

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

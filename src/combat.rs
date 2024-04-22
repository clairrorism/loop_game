use bevy::prelude::*;

#[derive(Event)]
pub struct DeathEvent(pub Entity);

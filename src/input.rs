use crate::{physics::prelude::*, Player};
use bevy::prelude::*;

#[derive(Event)]
pub enum PlayerAction {
    MoveRight,
    MoveLeft,
    Crouch,
    Jump,
    Attack,
    Interact,
}

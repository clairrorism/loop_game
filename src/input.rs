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

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<PlayerAction>,
) {
    let right = keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let left = keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let jump = keys.any_pressed([KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp]);
    let crouch = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::KeyS, KeyCode::ArrowDown]);

    if right ^ left {
        info!("meowww");
        events.send(if right {
            info!("owo");
            PlayerAction::MoveRight
        } else {
            PlayerAction::MoveLeft
        });
    }

    if jump {
        events.send(PlayerAction::Jump);
    }

    if crouch && !jump {
        events.send(PlayerAction::Crouch);
    }

    if keys.just_pressed(KeyCode::KeyE) {
        events.send(PlayerAction::Interact);
    }

    if mouse.just_pressed(MouseButton::Left) {
        events.send(PlayerAction::Attack);
    }
}

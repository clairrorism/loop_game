use bevy::prelude::*;

use crate::Player;
pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let (Ok(mut cam_xform), Ok(player_xform)) = (camera.get_single_mut(), player.get_single()) {
        cam_xform.translation = player_xform.translation;
    }
}

use bevy::prelude::*;

use crate::{Board, Box, Cursor, TILE_SIZE, TILE_SPACER};

//Handles movement of player cursor

pub fn move_cursor(
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Cursor>>,
    mut boxquery: Query<(&mut Box, &mut Transform), (Without<Cursor>, Without<Board>)>,
) {
    let mut cursor_transform: Mut<Transform> = query.single_mut();

    if key_input.just_pressed(KeyCode::ArrowUp) {
        if cursor_transform.translation.y == 220.0 {
            cursor_transform.translation.y = -330.0
        }

        cursor_transform.translation.y += TILE_SIZE + TILE_SPACER;
    } else if key_input.just_pressed(KeyCode::ArrowDown) {
        if cursor_transform.translation.y == -220.0 {
            cursor_transform.translation.y = 330.0
        }

        cursor_transform.translation.y -= TILE_SIZE + TILE_SPACER;
    } else if key_input.just_pressed(KeyCode::ArrowLeft) {
        if cursor_transform.translation.x == -220.0 {
            cursor_transform.translation.x = 330.0;
        }

        cursor_transform.translation.x -= TILE_SIZE + TILE_SPACER;
    } else if key_input.just_pressed(KeyCode::ArrowRight) {
        if cursor_transform.translation.x == 220.0 {
            cursor_transform.translation.x = -330.0;
        }

        cursor_transform.translation.x += TILE_SIZE + TILE_SPACER;
    }

    if key_input.just_pressed(KeyCode::Space) {
        for (boxes, mut boxt) in boxquery.iter_mut() {
            if (boxes.x == cursor_transform.translation.x)
                && (boxes.y == cursor_transform.translation.y)
            {
                boxt.translation.z = 3.0;
            }
        }
    }
}

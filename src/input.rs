use bevy::prelude::*;

use crate::{Board, Box, Cursor, Points, TILE_SIZE, TILE_SPACER};

//Handles movement of player cursor
pub fn move_cursor(
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Cursor>>,
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
}

//Handles box selection
pub fn box_select(
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Cursor>>,
    mut boxquery: Query<(&mut Box, &mut Visibility), (Without<Cursor>, Without<Board>)>,
    mut pointquery: Query<
        (&mut Points, &mut Text),
        (Without<Cursor>, Without<Board>, Without<Box>),
    >,
) {
    let cursor_transform: Mut<Transform> = query.single_mut();

    if key_input.just_pressed(KeyCode::Space) {
        let mut game_over: bool = false;
        for (mut boxes, mut boxv) in boxquery.iter_mut() {
            if (boxes.x == cursor_transform.translation.x)
                && (boxes.y == cursor_transform.translation.y)
            {
                if boxes.give_points == true {
                    *boxv = Visibility::Inherited;
                    boxes.give_points = false;
                    for (mut points, mut ptext) in pointquery.iter_mut() {
                        if boxes.value == 0 {
                            info!("GAME OVER: BOMB FLIPPED!");
                            game_over = true;
                        } else {
                            if points.val == 0 {
                                points.val += u64::from(boxes.value);
                            } else {
                                points.val *= u64::from(boxes.value);
                            }
                        }
                        *ptext = Text::from_sections([
                            TextSection::new(
                                "Score: ",
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                points.val.to_string(),
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                        ])
                    }
                } else {
                    info!("Error: Box already flipped!");
                }
            }
        }
        if game_over {
            for (mut boxes, mut boxv) in boxquery.iter_mut() {
                *boxv = Visibility::Inherited;
                boxes.give_points = false;
            }
        }
    }
}

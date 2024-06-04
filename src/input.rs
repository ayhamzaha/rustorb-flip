use bevy::prelude::*;

use crate::{
    board::Board, board::Box, board::Cursor, board::Points, board::TILE_SIZE, board::TILE_SPACER,
    mainmenu::Play, mainmenu::Quit, GameState,
};

pub fn mainmenu_input(
    key_input: Res<ButtonInput<KeyCode>>,
    mut playq: Query<
        (&mut Text, &mut Play),
        (
            With<Play>,
            Without<Quit>,
            Without<Points>,
            Without<Cursor>,
            Without<Board>,
            Without<Box>,
        ),
    >,
    mut quitq: Query<
        (&mut Text, &mut Quit),
        (
            With<Quit>,
            Without<Play>,
            Without<Points>,
            Without<Cursor>,
            Without<Board>,
            Without<Box>,
        ),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut playtext = playq.single_mut();
    let mut quittext = quitq.single_mut();

    if key_input.just_pressed(KeyCode::ArrowUp) {
        playtext.1.sel = true;
        quittext.1.sel = false;
        *playtext.0 = Text::from_section(
            "Play",
            TextStyle {
                font_size: 60.0,
                color: Color::YELLOW,
                ..default()
            },
        );
        *quittext.0 = Text::from_section(
            "Quit",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        );
    } else if key_input.just_pressed(KeyCode::ArrowDown) {
        playtext.1.sel = false;
        quittext.1.sel = true;
        *playtext.0 = Text::from_section(
            "Play",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        );
        *quittext.0 = Text::from_section(
            "Quit",
            TextStyle {
                font_size: 60.0,
                color: Color::YELLOW,
                ..default()
            },
        );
    }
    if key_input.just_pressed(KeyCode::Space) {
        if playtext.1.sel {
            info!("you chose play!");
            next_state.set(GameState::Playing);
        } else {
            info!("you chose quit!");
        }
    }
}

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
    mut next_state: ResMut<NextState<GameState>>,
) {
    let cursor_transform: Mut<Transform> = query.single_mut();

    if key_input.just_pressed(KeyCode::Space) {
        let mut game_over: bool = false;
        for (mut boxes, mut boxv) in boxquery.iter_mut() {
            if (boxes.x == cursor_transform.translation.x)
                && (boxes.y == cursor_transform.translation.y)
            {
                if boxes.give_points == true {
                    *boxv = Visibility::Visible;
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
                *boxv = Visibility::Visible;
                boxes.give_points = false;
            }
            next_state.set(GameState::MainMenu);
        }
    }
}

use bevy::prelude::*;

use crate::{
    board::{Board, Box, Cursor, GameMatrix, Nexter, TILE_SIZE, TILE_SPACER},
    colors,
    mainmenu::{Play, Quit},
    GameState, Points,
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
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
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
            next_state.set(GameState::Playing);
        } else {
            app_exit_events.send(bevy::app::AppExit);
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
    mut matr: Query<&mut GameMatrix, With<GameMatrix>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut nextq: Query<
        &mut Visibility,
        (
            With<Nexter>,
            Without<Quit>,
            Without<Play>,
            Without<Points>,
            Without<Cursor>,
            Without<Board>,
            Without<Box>,
        ),
    >,
) {
    let cursor_transform: Mut<Transform> = query.single_mut();

    if key_input.just_pressed(KeyCode::Space) {
        for (mut boxes, mut boxv) in boxquery.iter_mut() {
            if (boxes.x == cursor_transform.translation.x)
                && (boxes.y == cursor_transform.translation.y)
            {
                if boxes.give_points == true {
                    *boxv = Visibility::Visible;
                    boxes.give_points = false;
                    for (mut points, mut ptext) in pointquery.iter_mut() {
                        if boxes.value == 0 {
                            matr.single_mut().game_over = true;
                        } else {
                            if points.val == 0 {
                                points.val += u64::from(boxes.value);
                            } else {
                                points.val *= u64::from(boxes.value);
                            }
                        }
                        *ptext = Text::from_section(
                            points.val.to_string(),
                            TextStyle {
                                font_size: 30.0,
                                color: colors::FONT_COLOR,
                                ..default()
                            },
                        );
                    }
                }
            }
        }
    }
    if matr.single_mut().game_over {
        *nextq.single_mut() = Visibility::Visible;
        for (mut boxes, mut boxv) in boxquery.iter_mut() {
            *boxv = Visibility::Visible;
            boxes.give_points = false;
        }

        if key_input.just_pressed(KeyCode::Enter) {
            next_state.set(GameState::Lost);
        }
    }
    if pointquery.single().0.val == matr.single().target_score && !matr.single_mut().game_over {
        *nextq.single_mut() = Visibility::Visible;
        for (mut boxes, mut boxv) in boxquery.iter_mut() {
            *boxv = Visibility::Visible;
            boxes.give_points = false;
        }
        if key_input.just_pressed(KeyCode::Enter) {
            next_state.set(GameState::Won);
        }
    }
}

pub fn transition_handler(
    key_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        if current_state.get() == &GameState::Won {
            next_state.set(GameState::Playing);
        }
        if current_state.get() == &GameState::Lost {
            next_state.set(GameState::MainMenu);
        }
    }
}

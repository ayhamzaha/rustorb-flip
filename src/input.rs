use bevy::prelude::*;

use crate::{
    board::{self, Board, Box, Cursor, GameMatrix, Nexter, TimerBack, TILE_SIZE, TILE_SPACER},
    colors,
    mainmenu::{Play, Quit, Rules},
    GameState, Points,
};

pub fn rule_exit(
    key_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::MainMenu);
    }
}

pub fn mainmenu_input(
    key_input: Res<ButtonInput<KeyCode>>,
    mut playq: Query<
        (&mut Text, &mut Play),
        (
            With<Play>,
            Without<Quit>,
            Without<Rules>,
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
            Without<Rules>,
            Without<Play>,
            Without<Points>,
            Without<Cursor>,
            Without<Board>,
            Without<Box>,
        ),
    >,
    mut ruleq: Query<
        (&mut Text, &mut Rules),
        (
            With<Rules>,
            Without<Quit>,
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
    let mut ruletext = ruleq.single_mut();

    if playtext.1.sel {
        *playtext.0 = Text::from_section(
            "Play",
            TextStyle {
                font_size: 60.0,
                color: Color::YELLOW,
                ..default()
            },
        );
        *ruletext.0 = Text::from_section(
            "Rules",
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
                color: Color::WHITE,
                ..default()
            },
        );
    } else if ruletext.1.sel {
        *playtext.0 = Text::from_section(
            "Play",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        );
        *ruletext.0 = Text::from_section(
            "Rules",
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
    } else if quittext.1.sel {
        *playtext.0 = Text::from_section(
            "Play",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        );
        *ruletext.0 = Text::from_section(
            "Rules",
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

    if key_input.just_pressed(KeyCode::ArrowUp) {
        if playtext.1.sel {
            playtext.1.sel = false;
            quittext.1.sel = true;
            //quit is selected
        } else if ruletext.1.sel {
            ruletext.1.sel = false;
            playtext.1.sel = true;
            //play is selected
        } else if quittext.1.sel {
            quittext.1.sel = false;
            ruletext.1.sel = true;
            //rules is selected
        }
    } else if key_input.just_pressed(KeyCode::ArrowDown) {
        if playtext.1.sel {
            playtext.1.sel = false;
            ruletext.1.sel = true;
            quittext.1.sel = false;
        } else if ruletext.1.sel {
            ruletext.1.sel = false;
            quittext.1.sel = true;
            playtext.1.sel = false;
        } else if quittext.1.sel {
            quittext.1.sel = false;
            playtext.1.sel = true;
            ruletext.1.sel = false;
        }
    }
    if key_input.just_pressed(KeyCode::Space) {
        if playtext.1.sel {
            next_state.set(GameState::Playing);
        } else if ruletext.1.sel {
            next_state.set(GameState::Rules)
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
    mut timeq: Query<
        (&mut board::Timer, &mut Sprite, &mut Visibility),
        (
            With<board::Timer>,
            Without<Nexter>,
            Without<Quit>,
            Without<Play>,
            Without<Points>,
            Without<Cursor>,
            Without<Board>,
            Without<Box>,
            Without<TimerBack>,
        ),
    >,
    mut tbq: Query<
        &mut Visibility,
        (
            With<TimerBack>,
            Without<Nexter>,
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
    let mut time = timeq.single_mut();

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
                            time.0.time = time.0.perm_time;
                            //time.1.custom_size = Some(Vec2::new(time.0.time, 40.0));
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
        *tbq.single_mut() = Visibility::Hidden;
        *time.2 = Visibility::Hidden;
        time.0.perm_time = 60.0;
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
        *tbq.single_mut() = Visibility::Hidden;
        *time.2 = Visibility::Hidden;
        time.0.time = time.0.perm_time;
        for (mut boxes, mut boxv) in boxquery.iter_mut() {
            *boxv = Visibility::Visible;
            boxes.give_points = false;
        }
        if key_input.just_pressed(KeyCode::Enter) {
            next_state.set(GameState::Won);
        }
    }

    if !(matr.single_mut().game_over)
        && !(pointquery.single().0.val == matr.single().target_score
            && !matr.single_mut().game_over)
    {
        if time.0.time > 0.0 {
            time.0.time -= 0.01;
        } else {
            matr.single_mut().game_over = true;
        }
        let gap = time.0.time * (600.0 / time.0.perm_time);

        if gap >= 357.0 {
            *time.1 = Sprite {
                custom_size: Some(Vec2::new(time.0.time * (600.0 / time.0.perm_time), 40.0)),
                color: Color::hex("#66FF66").unwrap(),
                anchor: bevy::sprite::Anchor::CenterRight,
                ..default()
            };
        } else if gap < 356.0 && gap >= 194.0 {
            *time.1 = Sprite {
                custom_size: Some(Vec2::new(time.0.time * (600.0 / time.0.perm_time), 40.0)),
                color: Color::hex("#FFCC00").unwrap(),
                anchor: bevy::sprite::Anchor::CenterRight,
                ..default()
            };
        } else if gap < 193.0 {
            *time.1 = Sprite {
                custom_size: Some(Vec2::new(time.0.time * (600.0 / time.0.perm_time), 40.0)),
                color: Color::hex("#660000").unwrap(),
                anchor: bevy::sprite::Anchor::CenterRight,
                ..default()
            };
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

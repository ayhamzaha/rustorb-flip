use bevy::prelude::*;

mod board;
mod colors;
mod input;
mod mainmenu;

//cargo run --features bevy/dynamic_linking

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bomba Flip".into(),
                resizable: true,
                /*
                resize_constraints: WindowResizeConstraints {
                    min_width: 950.0,
                    min_height: 950.0,
                    max_width: 950.0,
                    max_height: 950.0,
                },
                */
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, cam_setup)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::MainMenu), mainmenu::set_mainmenu)
        .add_systems(
            Update,
            input::mainmenu_input.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), mainmenu::mainmenu_cleanup)
        .add_systems(
            OnEnter(GameState::Playing),
            (board::setup, board::spawn_board).chain(),
        )
        .add_systems(
            Update,
            (input::move_cursor, input::box_select).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), board::board_cleanup)
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Won,
    Lost,
}

fn cam_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        ..default()
    });
}

/*
//Runs on win
fn reset_game_board_win(
    mut commands: Commands,
    margbq: Query<
        Entity,
        (
            Without<board::Board>,
            With<board::Marginbox>,
            Without<board::Box>,
            Without<board::Cursor>,
            Without<board::Points>,
        ),
    >,
    mut boxq: Query<
        (&mut board::Box, Entity),
        (
            Without<board::Board>,
            Without<board::Marginbox>,
            With<board::Box>,
            Without<board::Cursor>,
            Without<board::Points>,
        ),
    >,
    boardq: Query<
        Entity,
        (
            With<board::Board>,
            Without<board::Marginbox>,
            Without<board::Box>,
            Without<board::Cursor>,
            Without<board::Points>,
        ),
    >,
    mut pointquery: Query<
        (&mut board::Points, &mut Text),
        (
            Without<board::Cursor>,
            Without<board::Board>,
            Without<board::Box>,
        ),
    >,
    mut levelq: Query<
        (&mut board::Level, &mut Text),
        (
            With<board::Level>,
            Without<board::Board>,
            Without<board::Marginbox>,
            Without<board::Box>,
            Without<board::Cursor>,
            Without<board::Points>,
        ),
    >,
    mut matrq: Query<
        (&mut board::GameMatrix, Entity),
        (
            With<board::GameMatrix>,
            Without<board::Level>,
            Without<board::Board>,
            Without<board::Marginbox>,
            Without<board::Box>,
            Without<board::Cursor>,
            Without<board::Points>,
        ),
    >,
) {

    let matrixq = matrq.single_mut();
    let player_points = pointquery.single();
    if matrixq.0.target_score == player_points.0.val {
        for (mut levelnum, mut leveltext) in levelq.iter_mut() {
            levelnum.lvl += 1;
            *leveltext = Text::from_sections([
                TextSection::new(
                    "Level: ",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                TextSection::new(
                    levelnum.lvl.to_string(),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
            ])
        }
        for ent in margbq.iter() {
            commands.entity(ent).despawn();
        }
        for (_boxes, ent) in boxq.iter_mut() {
            commands.entity(ent).despawn();
        }
        for ent in boardq.iter() {
            commands.entity(ent).despawn();
        }
        for (_game, ent) in matrq.iter() {
            commands.entity(ent).despawn();
        }
        for (points, mut ptext) in pointquery.iter_mut() {
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
    }
}
*/

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
        .add_systems(
            OnEnter(GameState::Lost),
            (board::board_cleanup_loss, loss_screen).chain(),
        )
        .add_systems(
            Update,
            input::transition_handler.run_if(in_state(GameState::Lost)),
        )
        .add_systems(OnExit(GameState::Lost), loss_screen_cleanup)
        .add_systems(
            OnEnter(GameState::Won),
            (board::board_cleanup_won, win_screen).chain(),
        )
        .add_systems(
            Update,
            input::transition_handler.run_if(in_state(GameState::Won)),
        )
        .add_systems(OnExit(GameState::Won), win_screen_cleanup)
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

#[derive(Resource)]
struct LossScreen {
    loss_screen: Entity,
}
#[derive(Resource)]
struct WinScreen {
    win_screen: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Level {
    pub lvl: u8,
}

#[derive(Component, Clone, Copy)]
pub struct Points {
    pub val: u64,
}
#[derive(Component, Clone, Copy)]
pub struct TotalPoints {
    pub total: u64,
}

fn cam_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        ..default()
    });

    //Sets and spawns points counter and point counter area
    let points = Points { val: 0 };
    let totalpoints = TotalPoints { total: 0 };
    let level = Level { lvl: 1 };
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                custom_size: Some(Vec2::new(200.0, 800.0)),
                ..default()
            },
            transform: Transform::from_xyz(-450.0, -50.0, 4.0),
            ..default()
        })
        //point counter
        .with_children(|builder| {
            builder
                .spawn(Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Score: ",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            points.val.to_string() + "\n",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                    ]),
                    transform: Transform::from_xyz(0.0, 250.0, 1.0),
                    ..default()
                })
                .insert(points);

            builder
                .spawn(Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Level: ",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            level.lvl.to_string(),
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                    ]),
                    transform: Transform::from_xyz(0.0, 150.0, 1.0),
                    ..default()
                })
                .insert(level);
        });

    commands
        .spawn(Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    "Total Score: ",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                TextSection::new(
                    totalpoints.total.to_string(),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
            ]),
            transform: Transform::from_xyz(0.0, 300.0, 1.0),
            ..default()
        })
        .insert(totalpoints);
}

fn loss_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let loss_screen = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(800.0, 800.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, -50.0, 0.0),
            texture: asset_server.load("game_backfinal.png"),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "YOU LOST!",
                    TextStyle {
                        font_size: 120.0,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 150.0, 1.0),
                ..default()
            });
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "Press [space] to continue...",
                    TextStyle {
                        font_size: 40.0,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        })
        .id();
    commands.insert_resource(LossScreen {
        loss_screen: loss_screen,
    });
}

fn loss_screen_cleanup(mut commands: Commands, loss_screen: Res<LossScreen>) {
    commands.entity(loss_screen.loss_screen).despawn_recursive();
}

fn win_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let win_screen = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(800.0, 800.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, -50.0, 0.0),
            texture: asset_server.load("game_backfinal.png"),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "YOU WON!",
                    TextStyle {
                        font_size: 120.0,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 150.0, 1.0),
                ..default()
            });
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "Press [space] to continue...",
                    TextStyle {
                        font_size: 40.0,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        })
        .id();
    commands.insert_resource(WinScreen {
        win_screen: win_screen,
    });
}

fn win_screen_cleanup(mut commands: Commands, win_screen: Res<WinScreen>) {
    commands.entity(win_screen.win_screen).despawn_recursive();
}

/* this is the reset on win code all needs a lot of work fuck my chungus life mannn
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

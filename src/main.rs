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

#[derive(Component, Clone, Copy)]
pub struct Scoreboard;

fn cam_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        ..default()
    });

    //Sets and spawns points counter and point counter area
    let points = Points { val: 0 };
    let totalpoints = TotalPoints { total: 0 };
    let level = Level { lvl: 1 };
    let scrbd = Scoreboard;
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200.0, 500.0)),
                ..default()
            },
            texture: asset_server.load("scrbd_back.png"),
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(-475.0, -50.0, 4.0),
            ..default()
        })
        .insert(scrbd)
        //point counter
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "Score",
                    TextStyle {
                        font_size: 20.0,
                        color: colors::FONT_COLOR,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 43.0, 0.0),
                ..default()
            });

            builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        points.val.to_string(),
                        TextStyle {
                            font_size: 30.0,
                            color: colors::FONT_COLOR,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 7.0, 1.0),
                    ..default()
                })
                .insert(points);

            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "Level",
                    TextStyle {
                        font_size: 20.0,
                        color: colors::FONT_COLOR,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 208.0, 0.0),
                ..default()
            });

            builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        level.lvl.to_string(),
                        TextStyle {
                            font_size: 30.0,
                            color: colors::FONT_COLOR,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 170.0, 1.0),
                    ..default()
                })
                .insert(level);

            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "Total Score",
                    TextStyle {
                        font_size: 20.0,
                        color: colors::FONT_COLOR,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, -140.0, 0.0),
                ..default()
            });

            builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        totalpoints.total.to_string(),
                        TextStyle {
                            font_size: 30.0,
                            color: colors::FONT_COLOR,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, -180.0, 1.0),
                    ..default()
                })
                .insert(totalpoints);
        });
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

fn loss_screen_cleanup(
    mut commands: Commands,
    loss_screen: Res<LossScreen>,
    mut scrbdq: Query<&mut Visibility, With<Scoreboard>>,
) {
    *scrbdq.single_mut() = Visibility::Hidden;
    commands.entity(loss_screen.loss_screen).despawn_recursive();
}

fn win_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let win_screen = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1100.0, 1100.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, 800.0, 0.0),
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

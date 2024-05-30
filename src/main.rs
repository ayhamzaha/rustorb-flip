use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;

mod colors;
mod input;

//cargo run --features bevy/dynamic_linking
//e

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
        .add_systems(Startup, (setup, spawn_board))
        .add_systems(
            Update,
            (
                input::move_cursor,
                input::box_select,
                (reset_game_board, spawn_board).run_if(input_just_pressed(KeyCode::KeyR)),
            ),
        )
        .run();
}

#[derive(Component)]
struct Points {
    val: u64,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //spawns our camera
    commands.spawn(Camera2dBundle::default());

    let cursor: Cursor = Cursor {
        start_pos_x: 0.0,
        start_pos_y: 0.0,
    };

    //Spawns the background of the game
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(800.0, 800.0)),
            ..default()
        },
        transform: Transform::from_xyz(50.0, -50.0, 0.0),
        texture: asset_server.load("game_backfinal.png"),
        ..default()
    });

    //Spawns the cursor
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                color: Color::GREEN,
                ..default()
            },
            transform: Transform::from_xyz(cursor.start_pos_x, cursor.start_pos_y, 6.0),
            texture: asset_server.load("selected_frame.png"),
            ..default()
        })
        .insert(cursor);

    //Sets and spawns points counter and point counter area
    let points = Points { val: 0 };
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
                            points.val.to_string(),
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
        });
}

const TILE_SIZE: f32 = 80.0;
const TILE_SPACER: f32 = 30.0;

#[derive(Component, Clone, Copy, Debug)]
struct Board {
    size: u8,
    physical_board_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_board_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_board_size,
        }
    }
}

#[derive(Component)]
struct Cursor {
    start_pos_x: f32,
    start_pos_y: f32,
}

#[derive(Component, Copy, Clone)]
struct Box {
    x: f32,
    y: f32,
    z: f32,
    give_points: bool,
    value: u8,
}

#[derive(Component, Clone, Copy, Debug)]
struct Marginbox;

#[derive(Component)]
struct GameMatrix {
    matr: [[u8; 5]; 5],
    two_bank: u8,
    three_bank: u8,
    r0_sum: u8,
    r1_sum: u8,
    r2_sum: u8,
    r3_sum: u8,
    r4_sum: u8,
    r0_bsum: u8,
    r1_bsum: u8,
    r2_bsum: u8,
    r3_bsum: u8,
    r4_bsum: u8,
    c0_sum: u8,
    c1_sum: u8,
    c2_sum: u8,
    c3_sum: u8,
    c4_sum: u8,
    c0_bsum: u8,
    c1_bsum: u8,
    c2_bsum: u8,
    c3_bsum: u8,
    c4_bsum: u8,
}

fn set_game_matrix() -> GameMatrix {
    let mut matr = GameMatrix {
        matr: [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        two_bank: 4,
        three_bank: 3,
        r0_sum: 0,
        r1_sum: 0,
        r2_sum: 0,
        r3_sum: 0,
        r4_sum: 0,
        r0_bsum: 0,
        r1_bsum: 0,
        r2_bsum: 0,
        r3_bsum: 0,
        r4_bsum: 0,
        c0_sum: 0,
        c1_sum: 0,
        c2_sum: 0,
        c3_sum: 0,
        c4_sum: 0,
        c0_bsum: 0,
        c1_bsum: 0,
        c2_bsum: 0,
        c3_bsum: 0,
        c4_bsum: 0,
    };

    for tile in (u8::from(0)..u8::from(5)).cartesian_product(u8::from(0)..u8::from(5)) {
        let val: u8 = rand::random::<u8>() % u8::from(4);
        match val {
            3 => {
                if matr.three_bank == 0 {
                    matr.matr[usize::from(tile.0)][usize::from(tile.1)] = 1;
                } else {
                    matr.three_bank -= 1;
                    matr.matr[usize::from(tile.0)][usize::from(tile.1)] = 3;
                }
            }
            2 => {
                if matr.two_bank == 0 {
                    matr.matr[usize::from(tile.0)][usize::from(tile.1)] = 0;
                } else {
                    matr.two_bank -= 1;
                    matr.matr[usize::from(tile.0)][usize::from(tile.1)] = 2;
                }
            }
            1 => {
                matr.matr[usize::from(tile.0)][usize::from(tile.1)] = 1;
            }
            0 => {
                matr.matr[usize::from(tile.0)][usize::from(tile.1)] = 0;
            }
            _ => {
                matr.matr[usize::from(tile.0)][usize::from(tile.1)] = 0;
            }
        }
    }

    for i in 0..matr.matr.len() {
        for j in 0..matr.matr.len() {
            if i == 0 {
                matr.c0_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.c0_bsum += 1;
                }
            } else if i == 1 {
                matr.c1_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.c1_bsum += 1;
                }
            } else if i == 2 {
                matr.c2_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.c2_bsum += 1;
                }
            } else if i == 3 {
                matr.c3_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.c3_bsum += 1;
                }
            } else if i == 4 {
                matr.c4_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.c4_bsum += 1;
                }
            }

            if j == 0 {
                matr.r0_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.r0_bsum += 1;
                }
            } else if j == 1 {
                matr.r1_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.r1_bsum += 1;
                }
            } else if j == 2 {
                matr.r2_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.r2_bsum += 1;
                }
            } else if j == 3 {
                matr.r3_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.r3_bsum += 1;
                }
            } else if j == 4 {
                matr.r4_sum += matr.matr[i][j];
                if matr.matr[i][j] == 0 {
                    matr.r4_bsum += 1;
                }
            }
        }
    }

    matr
}

fn reset_game_board(
    mut commands: Commands,
    margbq: Query<
        Entity,
        (
            Without<Board>,
            With<Marginbox>,
            Without<Box>,
            Without<Cursor>,
            Without<Points>,
        ),
    >,
    boxq: Query<
        Entity,
        (
            Without<Board>,
            Without<Marginbox>,
            With<Box>,
            Without<Cursor>,
            Without<Points>,
        ),
    >,
    boardq: Query<
        Entity,
        (
            With<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Cursor>,
            Without<Points>,
        ),
    >,
) {
    for ent in margbq.iter() {
        commands.entity(ent).despawn();
    }
    for ent in boxq.iter() {
        commands.entity(ent).despawn();
    }
    for ent in boardq.iter() {
        commands.entity(ent).despawn();
    }
}

//Spawns game board
fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let board = Board::new(5);
    let offset = -(board.physical_board_size) / 2.0 + TILE_SIZE * 0.5;
    let tab_size = board.physical_board_size - (TILE_SPACER * 2.0);
    let margbox = Marginbox;

    let matr = set_game_matrix();

    //spawns board with grid
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    board.physical_board_size,
                    board.physical_board_size,
                )),
                ..default()
            },
            texture: asset_server.load("gameback.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(board)
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                let mut boxes = Box {
                    x: offset + f32::from(tile.0) * TILE_SIZE + f32::from(tile.0 + 1) * TILE_SPACER,
                    y: offset + f32::from(tile.1) * TILE_SIZE + f32::from(tile.1 + 1) * TILE_SPACER,
                    z: 2.0,
                    give_points: true,
                    value: matr.matr[usize::from(tile.0)][usize::from(tile.1)],
                };
                boxes.value = matr.matr[usize::from(tile.0)][usize::from(tile.1)];

                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },

                        transform: Transform::from_xyz(boxes.x, boxes.y, 2.0),
                        texture: asset_server.load("box4.png"),
                        ..default()
                    })
                    //Spawns items under the boxes
                    .with_children(|builder| {
                        match matr.matr[usize::from(tile.0)][usize::from(tile.1)] {
                            0 => {
                                builder
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(40.0, 40.0)),
                                            ..default()
                                        },
                                        transform: Transform::from_xyz(0.0, 0.0, boxes.z),
                                        visibility: Visibility::Hidden,

                                        texture: asset_server.load("bomba.png"),
                                        ..default()
                                    })
                                    .insert(boxes);
                            }
                            1 => {
                                builder
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(20.0, 20.0)),

                                            ..default()
                                        },
                                        transform: Transform::from_xyz(0.0, 0.0, boxes.z),
                                        visibility: Visibility::Hidden,
                                        texture: asset_server.load("one.png"),
                                        ..default()
                                    })
                                    .insert(boxes);
                            }
                            2 => {
                                builder
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(20.0, 20.0)),
                                            ..default()
                                        },
                                        transform: Transform::from_xyz(0.0, 0.0, boxes.z),
                                        visibility: Visibility::Hidden,

                                        texture: asset_server.load("twoo.png"),
                                        ..default()
                                    })
                                    .insert(boxes);
                            }
                            3 => {
                                builder
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(20.0, 20.0)),

                                            ..default()
                                        },
                                        transform: Transform::from_xyz(0.0, 0.0, boxes.z),
                                        visibility: Visibility::Hidden,

                                        texture: asset_server.load("threee.png"),
                                        ..default()
                                    })
                                    .insert(boxes);
                            }
                            _ => {
                                builder
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(20.0, 20.0)),

                                            ..default()
                                        },
                                        transform: Transform::from_xyz(0.0, 0.0, boxes.z),
                                        visibility: Visibility::Hidden,
                                        ..default()
                                    })
                                    .insert(boxes);
                            }
                        };
                    })
                    .insert(margbox);
                //Spawns grid box connectors (horizontal)
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: colors::BOX_COLOR_ARRAY[usize::from(4 - tile.1)],
                            custom_size: Some(Vec2::new(TILE_SPACER * 2.0 - 10.0, TILE_SIZE / 5.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (offset + TILE_SPACER * 2.0)
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            offset
                                + f32::from(tile.1) * TILE_SIZE
                                + f32::from(tile.1 + 1) * TILE_SPACER,
                            1.0,
                        ),
                        texture: asset_server.load("connectors_horiz.png"),
                        ..default()
                    })
                    .insert(margbox);

                //Spawns grid box connectors (vertical)
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: colors::BOX_COLOR_ARRAY[usize::from(tile.0)],
                            custom_size: Some(Vec2::new(TILE_SIZE / 5.0, TILE_SPACER * 2.0 - 10.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            offset
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            (offset - TILE_SPACER * 2.0)
                                + f32::from(tile.1) * TILE_SIZE
                                + f32::from(tile.1 + 1) * TILE_SPACER,
                            1.0,
                        ),
                        texture: asset_server.load("connectors_vertical.png"),
                        ..default()
                    })
                    .insert(margbox);
            }
        });

    //Spawns bottom info tab with grid
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(tab_size + TILE_SPACER * 2.0, TILE_SIZE + 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, offset - (TILE_SIZE + 15.0), 0.0),
            texture: asset_server.load("slab_back.png"),
            ..default()
        })
        .insert(margbox)
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: colors::BOX_COLOR_ARRAY[usize::from(tile.0)],
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            offset
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            0.0,
                            1.0,
                        ),
                        texture: asset_server.load("outlinebox_vertical.png"),
                        ..default()
                    })
                    .insert(margbox);
            }
        })
        .insert(margbox)
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                //Spawns coin image
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(24.0, 24.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (offset - 15.0)
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            -15.0,
                            3.0,
                        ),
                        texture: asset_server.load("coin.png"),
                        ..default()
                    })
                    .insert(margbox);

                builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            match tile.0 {
                                0 => matr.c0_sum.to_string(),
                                1 => matr.c1_sum.to_string(),
                                2 => matr.c2_sum.to_string(),
                                3 => matr.c3_sum.to_string(),
                                4 => matr.c4_sum.to_string(),
                                _ => String::from("--"),
                            },
                            TextStyle {
                                font_size: 22.0,
                                color: colors::FONT_COLOR,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(
                            (offset + 13.0)
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            -15.0,
                            13.0,
                        ),
                        ..default()
                    })
                    .insert(margbox);

                //Spawns bomb image
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(24.0, 24.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (offset - 15.0)
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            15.0,
                            3.0,
                        ),
                        texture: asset_server.load("bomba.png"),
                        ..default()
                    })
                    .insert(margbox);

                builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            match tile.0 {
                                0 => matr.c0_bsum.to_string(),
                                1 => matr.c1_bsum.to_string(),
                                2 => matr.c2_bsum.to_string(),
                                3 => matr.c3_bsum.to_string(),
                                4 => matr.c4_bsum.to_string(),
                                _ => String::from("--"),
                            },
                            TextStyle {
                                font_size: 22.0,
                                color: colors::FONT_COLOR,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(
                            (offset + 13.0)
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            16.0,
                            13.0,
                        ),
                        ..default()
                    })
                    .insert(margbox);
            }
        })
        .insert(margbox);

    //Spawns right info tab with grid
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::hex(colors::BCKGRD_HEX_STRING).unwrap(),
                custom_size: Some(Vec2::new(TILE_SIZE + 30.0, tab_size + TILE_SPACER * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(-offset + (TILE_SIZE + 15.0), 0.0, 0.0),
            texture: asset_server.load("slab_back_vert.png"),
            ..default()
        })
        .insert(margbox)
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: colors::BOX_COLOR_ARRAY[usize::from(4 - tile.0)],
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            0.0,
                            offset
                                + f32::from(tile.0) * TILE_SIZE
                                + f32::from(tile.0 + 1) * TILE_SPACER,
                            2.0,
                        ),
                        texture: asset_server.load("outlinebox_horiz.png"),
                        ..default()
                    })
                    .insert(margbox);
            }
        })
        .insert(margbox)
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                //Spawns coin image
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(24.0, 24.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            -15.0,
                            (offset - 15.0)
                                + f32::from(tile.1) * TILE_SIZE
                                + f32::from(tile.1 + 1) * TILE_SPACER,
                            3.0,
                        ),
                        texture: asset_server.load("coin.png"),
                        ..default()
                    })
                    .insert(margbox);

                builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            match tile.1 {
                                0 => matr.r0_sum.to_string(),
                                1 => matr.r1_sum.to_string(),
                                2 => matr.r2_sum.to_string(),
                                3 => matr.r3_sum.to_string(),
                                4 => matr.r4_sum.to_string(),
                                _ => String::from("--"),
                            },
                            TextStyle {
                                font_size: 22.0,
                                color: colors::FONT_COLOR,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(
                            13.0,
                            (offset - 15.0)
                                + f32::from(tile.1) * TILE_SIZE
                                + f32::from(tile.1 + 1) * TILE_SPACER,
                            3.0,
                        ),
                        ..default()
                    })
                    .insert(margbox);

                //Spawns bomb image
                builder
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(24.0, 24.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            -15.0,
                            (offset + 15.0)
                                + f32::from(tile.1) * TILE_SIZE
                                + f32::from(tile.1 + 1) * TILE_SPACER,
                            3.0,
                        ),
                        texture: asset_server.load("bomba.png"),
                        ..default()
                    })
                    .insert(margbox);

                builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            match tile.1 {
                                0 => matr.r0_bsum.to_string(),
                                1 => matr.r1_bsum.to_string(),
                                2 => matr.r2_bsum.to_string(),
                                3 => matr.r3_bsum.to_string(),
                                4 => matr.r4_bsum.to_string(),
                                _ => String::from("--"),
                            },
                            TextStyle {
                                font_size: 22.0,
                                color: colors::FONT_COLOR,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(
                            13.0,
                            (offset + 15.0)
                                + f32::from(tile.1) * TILE_SIZE
                                + f32::from(tile.1 + 1) * TILE_SPACER,
                            3.0,
                        ),
                        ..default()
                    })
                    .insert(margbox);
            }
        })
        .insert(margbox);
}

use bevy::prelude::*;
use itertools::Itertools;

use crate::{colors, Level, Points, TotalPoints};

pub const TILE_SIZE: f32 = 80.0;
pub const TILE_SPACER: f32 = 30.0;

#[derive(Component, Clone, Copy, Debug)]
pub struct Board {
    pub size: u8,
    pub physical_board_size: f32,
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
pub struct Cursor {
    pub start_pos_x: f32,
    pub start_pos_y: f32,
}

#[derive(Component, Copy, Clone)]
pub struct Box {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub give_points: bool,
    pub value: u8,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Marginbox;

#[derive(Component, Clone, Copy, Debug)]
pub struct Nexter;

#[derive(Component, Copy, Clone)]
pub struct GameMatrix {
    pub matr: [[u8; 5]; 5],
    pub target_score: u64,
    pub num_bank: u8,
    pub game_over: bool,
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

pub fn set_game_matrix(level: Level) -> GameMatrix {
    let mut matr = GameMatrix {
        matr: [
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
        ],
        num_bank: 4,
        target_score: 1,
        game_over: false,
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
    matr.num_bank += level.lvl / 3;
    for tile in (u8::from(0)..u8::from(5)).cartesian_product(u8::from(0)..u8::from(5)) {
        let val: u8 = rand::random::<u8>() % u8::from(2);
        match val {
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
    for _i in 0..matr.num_bank - 1 {
        let choice = rand::random::<u8>() % u8::from(2);
        let mut rand_x = rand::random::<u8>() % u8::from(5);
        let mut rand_y = rand::random::<u8>() % u8::from(5);
        while matr.matr[usize::from(rand_x)][usize::from(rand_y)] == 3
            || matr.matr[usize::from(rand_x)][usize::from(rand_y)] == 2
        {
            rand_x = rand::random::<u8>() % u8::from(5);
            rand_y = rand::random::<u8>() % u8::from(5);
        }
        match choice {
            1 => {
                matr.matr[usize::from(rand_x)][usize::from(rand_y)] = 3;
                matr.target_score *= 3;
            }
            0 => {
                matr.matr[usize::from(rand_x)][usize::from(rand_y)] = 2;
                matr.target_score *= 2;
            }
            _ => {}
        }
    }

    for i in 0..matr.matr.len() {
        for j in 0..matr.matr.len() {
            match i {
                4 => {
                    matr.c4_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.c4_bsum += 1;
                    }
                }
                3 => {
                    matr.c3_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.c3_bsum += 1;
                    }
                }
                2 => {
                    matr.c2_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.c2_bsum += 1;
                    }
                }
                1 => {
                    matr.c1_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.c1_bsum += 1;
                    }
                }
                0 => {
                    matr.c0_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.c0_bsum += 1;
                    }
                }
                _ => {}
            }

            match j {
                4 => {
                    matr.r4_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.r4_bsum += 1;
                    }
                }
                3 => {
                    matr.r3_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.r3_bsum += 1;
                    }
                }
                2 => {
                    matr.r2_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.r2_bsum += 1;
                    }
                }
                1 => {
                    matr.r1_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.r1_bsum += 1;
                    }
                }
                0 => {
                    matr.r0_sum += matr.matr[i][j];
                    if matr.matr[i][j] == 0 {
                        matr.r0_bsum += 1;
                    }
                }
                _ => {}
            }
        }
    }

    matr
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        transform: Transform::from_xyz(50.0, -80.0, 0.0),
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
}

//Spawns game board
pub fn spawn_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut levelq: Query<
        &mut Level,
        (
            With<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Cursor>,
            Without<Points>,
        ),
    >,
) {
    let board = Board::new(5);
    let offset = -(board.physical_board_size) / 2.0 + TILE_SIZE * 0.5;
    let tab_size = board.physical_board_size - (TILE_SPACER * 2.0);
    let margbox = Marginbox;
    let levelnum = levelq.single_mut();
    let matr = set_game_matrix(Level { lvl: levelnum.lvl });
    let nexter = Nexter {};

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.0, 0.0)),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(matr);
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
            builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "PRESS ENTER TO CONTINUE",
                        TextStyle {
                            font_size: 40.0,
                            color: colors::FONT_COLOR,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(50.0, 300.0, 1.0),
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(nexter);
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
                    });

                //Spawns grid box connectors (horizontal)
                builder.spawn(SpriteBundle {
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
                });

                //Spawns grid box connectors (vertical)
                builder.spawn(SpriteBundle {
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
                });
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
                builder.spawn(SpriteBundle {
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
                });
            }
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                //Spawns coin image
                builder.spawn(SpriteBundle {
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
                });

                builder.spawn(Text2dBundle {
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
                });

                //Spawns bomb image
                builder.spawn(SpriteBundle {
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
                });

                builder.spawn(Text2dBundle {
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
                });
            }
        });

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
                builder.spawn(SpriteBundle {
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
                });
            }
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                //Spawns coin image
                builder.spawn(SpriteBundle {
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
                });

                builder.spawn(Text2dBundle {
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
                });

                //Spawns bomb image
                builder.spawn(SpriteBundle {
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
                });

                builder.spawn(Text2dBundle {
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
                });
            }
        });
}

//Runs on loss or restart(r button press)
pub fn board_cleanup_loss(
    mut commands: Commands,
    margbq: Query<
        Entity,
        (
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            With<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    boardq: Query<
        Entity,
        (
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            With<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    mut pointquery: Query<
        (&mut Text, &mut Points),
        (
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            With<Points>,
            Without<TotalPoints>,
        ),
    >,
    mut levelq: Query<
        (&mut Text, &mut Level),
        (
            Without<Cursor>,
            Without<GameMatrix>,
            With<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    mut totalpq: Query<
        (&mut Text, &mut TotalPoints),
        (
            With<TotalPoints>,
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
        ),
    >,
    cursq: Query<
        Entity,
        (
            With<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    matrq: Query<
        Entity,
        (
            Without<Cursor>,
            With<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
) {
    let mut points = pointquery.single_mut();
    let mut level = levelq.single_mut();
    let mut totalpoints = totalpq.single_mut();

    totalpoints.1.total = 0;
    points.1.val = 0;
    level.1.lvl = 1;

    *points.0 = Text::from_section(
        points.1.val.to_string(),
        TextStyle {
            font_size: 30.0,
            color: colors::FONT_COLOR,
            ..default()
        },
    );

    *level.0 = Text::from_section(
        level.1.lvl.to_string(),
        TextStyle {
            font_size: 30.0,
            color: colors::FONT_COLOR,
            ..default()
        },
    );

    *totalpoints.0 = Text::from_section(
        totalpoints.1.total.to_string(),
        TextStyle {
            font_size: 30.0,
            color: colors::FONT_COLOR,
            ..default()
        },
    );

    for ent in margbq.iter() {
        commands.entity(ent).despawn_recursive();
    }
    commands.entity(boardq.single()).despawn_recursive();
    commands.entity(cursq.single()).despawn();
    commands.entity(matrq.single()).despawn();
}

pub fn board_cleanup_won(
    mut commands: Commands,
    margbq: Query<
        Entity,
        (
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            With<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    boardq: Query<
        Entity,
        (
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            With<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    mut pointquery: Query<
        (&mut Text, &mut Points),
        (
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            With<Points>,
            Without<TotalPoints>,
        ),
    >,
    mut levelq: Query<
        (&mut Text, &mut Level),
        (
            Without<Cursor>,
            Without<GameMatrix>,
            With<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    mut totalpq: Query<
        (&mut Text, &mut TotalPoints),
        (
            With<TotalPoints>,
            Without<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
        ),
    >,
    cursq: Query<
        Entity,
        (
            With<Cursor>,
            Without<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
    matrq: Query<
        Entity,
        (
            Without<Cursor>,
            With<GameMatrix>,
            Without<Level>,
            Without<Board>,
            Without<Marginbox>,
            Without<Box>,
            Without<Points>,
            Without<TotalPoints>,
        ),
    >,
) {
    let mut points = pointquery.single_mut();
    let mut level = levelq.single_mut();
    let mut totalpoints = totalpq.single_mut();

    totalpoints.1.total += points.1.val;
    points.1.val = 0;
    level.1.lvl += 1;

    *points.0 = Text::from_section(
        points.1.val.to_string(),
        TextStyle {
            font_size: 30.0,
            color: colors::FONT_COLOR,
            ..default()
        },
    );

    *level.0 = Text::from_section(
        level.1.lvl.to_string(),
        TextStyle {
            font_size: 30.0,
            color: colors::FONT_COLOR,
            ..default()
        },
    );

    *totalpoints.0 = Text::from_section(
        totalpoints.1.total.to_string(),
        TextStyle {
            font_size: 30.0,
            color: colors::FONT_COLOR,
            ..default()
        },
    );

    for ent in margbq.iter() {
        commands.entity(ent).despawn_recursive();
    }
    commands.entity(boardq.single()).despawn_recursive();
    commands.entity(cursq.single()).despawn();
    commands.entity(matrq.single()).despawn();
}

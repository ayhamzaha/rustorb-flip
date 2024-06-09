use bevy::prelude::*;

use crate::Scoreboard;

#[derive(Component, Debug)]
pub struct Play {
    pub sel: bool,
}

#[derive(Component)]
pub struct Quit {
    pub sel: bool,
}

#[derive(Resource)]
pub struct Menu {
    menu: Entity,
}

pub fn set_mainmenu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let play = Play { sel: true };
    let quit = Quit { sel: false };
    let menu_ent = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(800.0, 800.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, -50.0, 0.0),
            texture: asset_server.load("gameback.png"),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "BOMBA FLIP",
                    TextStyle {
                        font_size: 120.0,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 215.0, 1.0),
                ..default()
            });

            builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "Play",
                        TextStyle {
                            font_size: 60.0,
                            color: Color::YELLOW,
                            ..default()
                        },
                    ),

                    transform: Transform::from_xyz(0.0, -20.0, 1.0),
                    ..default()
                })
                .insert(play);
            builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "Quit",
                        TextStyle {
                            font_size: 60.0,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, -120.0, 1.0),
                    ..default()
                })
                .insert(quit);
        })
        .id();
    commands.insert_resource(Menu { menu: menu_ent });
}

pub fn mainmenu_cleanup(
    mut commands: Commands,
    menu_data: Res<Menu>,
    mut scrbdq: Query<&mut Visibility, With<Scoreboard>>,
) {
    *scrbdq.single_mut() = Visibility::Visible;
    commands.entity(menu_data.menu).despawn_recursive();
}

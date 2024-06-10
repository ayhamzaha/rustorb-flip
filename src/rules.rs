use ::bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::Scoreboard;

/*
Should teach the player how to play the game by explaining every input (arrow keys, space bar, and enter key).
Should explain the game and the games loop (what the boxes on the margins mean and how score is calculated),
(the timer and how it works),(the games looping and increasing difficulty).


Using the colored boxes you can determine how many coins and bombs there are in each row/column. Use that information
to flip boxes that have either 1, 2, or 3 and avoid any bombs. Be mindful of the timer at the top of the screen, if you
successfully flip a box that is not a bomb the timer will reset. However if you flip a bomb or the timer reaches 0, you
lose and the game resets. Once you have flipped every 2 and 3 box the game will end, and you will be moved to the next
level. Your score for that level will be added to your total score. Each level increases the difficulty by adding more
2 and 3 boxes and making the timer faster.
*/

#[derive(Component, Clone, Copy)]
pub struct Rules;

pub fn set_rules(mut commands: Commands, mut scrbdq: Query<&mut Visibility, (With<Scoreboard>,)>) {
    *scrbdq.single_mut() = Visibility::Hidden;
    let rules = Rules;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "How To Play",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, 300.0, 1.0),
            ..default()
        })
        .insert(rules);

    commands.spawn(Text2dBundle {
        text: Text::from_sections([
            TextSection::new(
                "\t Using the colored boxes you can see how many coins and bombs there are in each row/column.\n 
\t Use that information to flip boxes that have either 1, 2, or 3 and avoid any bombs. Be mindful of the timer at the top of the screen, if you flip a box that is not a bomb the timer will reset. However if you flip a bomb or the timer reaches 0, you lose and the game will restart.\n 
\t Once you have flipped every 2 and 3 box the game will end, and you will be moved to the next level. Your score for that level will be added to your total score. Each level increases the difficulty by adding more 2 and 3 boxes and making the timer faster. Press enter to return...",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
        ]),
        text_2d_bounds: Text2dBounds {
            size: Vec2::new(600.0, 600.0),
            ..default()
        },
        text_anchor: bevy::sprite::Anchor::TopCenter,
        transform: Transform::from_xyz(0.0, 240.0, 1.0),
        ..default()
    }).insert(rules);
}

pub fn rules_cleanup(mut commands: Commands, ruleq: Query<Entity, With<Rules>>) {
    for ent in ruleq.iter() {
        commands.entity(ent).despawn();
    }
}

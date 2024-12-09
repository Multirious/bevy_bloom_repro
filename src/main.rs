use bevy::core_pipeline::{bloom::Bloom, tonemapping::Tonemapping};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..Default::default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
    ));

    let white: Color = (Srgba::WHITE * 2.).into();
    let blue: Color = (Srgba::rgb_u8(103, 163, 217) * 5.).into();
    let pink: Color = (Srgba::rgb_u8(248, 183, 205) * 5.).into();

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            color: white,
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            color: blue,
            ..default()
        },
        Transform::from_xyz(40., 20., -1.),
    ));
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            color: pink,
            ..default()
        },
        Transform::from_xyz(-40., -20., 1.),
    ));
}

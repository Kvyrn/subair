mod player;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "subair".into(),
                mode: bevy::window::WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(bevy::window::close_on_esc)
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.05, 0.0, 0.2)))
        .add_startup_system(basic_scene)
        .run();
}

fn basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(100.0, 1.0, 100.0).into()),
        material: materials.add(Color::AQUAMARINE.into()),
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
}

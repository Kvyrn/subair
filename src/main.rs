mod player;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player::PlayerPlugin;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(bevy::window::close_on_esc)
        .add_plugin(PlayerPlugin)
        .add_startup_system(basic_scene)
        .add_system(rotate)
        .run();
}

#[derive(Debug, Component)]
struct Rotate;

fn basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cam = commands.spawn(Camera3dBundle::default()).id();

    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(100.0, 1.0, 100.0).into()),
        material: materials.add(Color::AQUAMARINE.into()),
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("player_body.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, -3.0),
            ..default()
        })
        .insert(Rotate);
}

fn rotate(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut transform in q.iter_mut() {
        transform.rotate_y(time.delta_seconds() * 2.0 * PI / 5.0);
    }
}

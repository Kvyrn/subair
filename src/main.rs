mod player;
mod world;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use player::PlayerPlugin;
use world::WorldPlugin;

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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(bevy::window::close_on_esc.after(capture_cursor))
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldPlugin)
        .insert_resource(ClearColor(Color::rgb(0.05, 0.0, 0.2)))
        .add_startup_system(basic_scene)
        .add_system(capture_cursor)
        .run();
}

fn capture_cursor(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for mut win in windows.iter_mut() {
            win.cursor.visible = false;
            win.cursor.grab_mode = CursorGrabMode::Locked;
        }
    }
    if key.any_pressed([KeyCode::Escape, KeyCode::Q]) {
        for mut win in windows.iter_mut() {
            win.cursor.visible = true;
            win.cursor.grab_mode = CursorGrabMode::None
        }
    }
}

fn basic_scene(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(shape::Box::new(100.0, 1.0, 100.0).into()),
    //         material: materials.add(Color::AQUAMARINE.into()),
    //         transform: Transform::from_xyz(0.0, -5.0, 0.0),
    //         ..default()
    //     })
    //     .insert((RigidBody::Fixed, Collider::cuboid(50.0, 0.5, 50.0)));
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

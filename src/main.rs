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

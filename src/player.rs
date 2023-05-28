use std::f32::consts::PI;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_system(register_propeller)
            .add_startup_system(spawn_player)
            .add_system(rotate_propeller);
    }
}

#[derive(Debug, Component, Reflect)]
struct Player {
    body_color: Color,
    band_color: Color,
}

#[derive(Debug, Component)]
struct Propeller;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cam = commands.spawn(Camera3dBundle::default()).id();
    commands.spawn(SceneBundle {
        scene: asset_server.load("player.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -3.0),
        ..default()
    });
}

fn register_propeller(
    query: Query<(Entity, &Name), (Without<Propeller>, Added<Name>)>,
    mut commands: Commands,
) {
    for (entity, name) in query.iter() {
        if name.as_str() == "Propeller" {
            commands.entity(entity).insert(Propeller);
            info!("Registered propeller");
        }
    }
}

fn rotate_propeller(mut query: Query<&mut Transform, With<Propeller>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        // One rotation per second
        transform.rotate_x(time.delta_seconds() * 2.0 * PI);
    }
}

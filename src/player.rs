use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;
use tracing::info;

const SENSITIVITY: f32 = 0.05;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<CalculatedInput>()
            .register_type::<Controlled>()
            .insert_resource(CalculatedInput::default())
            .add_plugin(ResourceInspectorPlugin::<CalculatedInput>::new())
            .add_system(register_propeller)
            .add_startup_system(spawn_player)
            .add_system(update_input)
            .add_system(calculate_rotation.after(update_input))
            .add_system(movement.after(calculate_rotation))
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

#[derive(Debug, Resource, Reflect, Default)]
struct CalculatedInput {
    vertical: f32,
    horizontal: f32,
    forward: f32,
}

#[derive(Debug, Component, Reflect, Default)]
struct Controlled {
    pitch: f32,
    yaw: f32,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Controlled::default())
        .insert(SpatialBundle::default())
        .insert((
            RigidBody::KinematicPositionBased,
            Collider::ball(0.8),
            // Collider::capsule_z(0.8, 0.5),
            KinematicCharacterController::default(),
            Velocity::default(),
        ))
        .with_children(|b| {
            b.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
            b.spawn(SceneBundle {
                scene: asset_server.load("player.glb#Scene0"),
                transform: Transform::from_rotation(Quat::from_rotation_y(PI / -2.0)),
                ..default()
            });
            b.spawn(SpotLightBundle {
                spot_light: SpotLight {
                    intensity: 2000.0,
                    range: 100.0,
                    outer_angle: PI / 6.0,
                    color: Color::rgb(1.0, 0.9, 0.7),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });
        });
}

fn update_input(
    keys: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    mut calcd: ResMut<CalculatedInput>,
) {
    let forward = keys.pressed(KeyCode::W);
    let backward = keys.pressed(KeyCode::S);
    let forward = match (forward, backward) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };
    let mut mouse_delta = Vec2::ZERO;
    for event in mouse.iter() {
        mouse_delta += event.delta;
    }

    calcd.forward = forward;
    calcd.horizontal = -mouse_delta.x * SENSITIVITY;
    calcd.vertical = -mouse_delta.y * SENSITIVITY;
}

fn calculate_rotation(
    input: Res<CalculatedInput>,
    mut query: Query<&mut Controlled>,
    time: Res<Time>,
) {
    for mut controlled in query.iter_mut() {
        controlled.pitch += input.vertical * time.delta_seconds();
        controlled.yaw += input.horizontal * time.delta_seconds();
        controlled.pitch = wrap_rotation(controlled.pitch);
        controlled.yaw = wrap_rotation(controlled.yaw);
    }
}

fn wrap_rotation(mut rot: f32) -> f32 {
    while rot > (PI * 2.0) {
        rot -= PI * 2.0;
    }
    while rot < (PI * 2.0) {
        rot += PI * 2.0;
    }
    rot
}

fn movement(
    mut query: Query<(
        &mut KinematicCharacterController,
        &mut Transform,
        &Controlled,
    )>,
    input: Res<CalculatedInput>,
    time: Res<Time>,
) {
    for (mut controller, mut transform, controlled) in query.iter_mut() {
        transform.rotation =
            Quat::from_rotation_y(controlled.yaw) * Quat::from_rotation_x(controlled.pitch);
        let trans = transform.forward() * time.delta_seconds() * 20.0 * input.forward;
        controller.translation = Some(trans);
    }
}

#[allow(clippy::type_complexity)]
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

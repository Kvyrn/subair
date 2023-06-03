mod generate;
mod marching_cubes_tables;
mod normals;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_rapier3d::prelude::*;
use futures_lite::future::{block_on, poll_once};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WorldInfo>()
            .add_plugin(ResourceInspectorPlugin::<WorldInfo>::new())
            .insert_resource(WorldInfo {
                seed: 23478235784239483,
            })
            .add_startup_system(schedule_world_gen)
            .add_startup_system(setup)
            .add_system(collect_world_mesh);
    }
}

#[derive(Debug, Reflect, Resource)]
pub struct WorldInfo {
    seed: u64,
}

#[derive(Debug, Resource)]
struct WorldMaterial(Handle<StandardMaterial>);

#[derive(Component)]
pub struct WorldMeshTask(Task<(Mesh, Collider)>);

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let handle = materials.add(StandardMaterial {
        base_color: Color::ORANGE_RED,
        ..default()
    });
    commands.insert_resource(WorldMaterial(handle));
}

fn schedule_world_gen(info: Res<WorldInfo>, mut commands: Commands) {
    let pool = AsyncComputeTaskPool::get();
    let task = pool.spawn(generate::generate_world(info.seed));
    commands.spawn(WorldMeshTask(task));
}

fn collect_world_mesh(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut WorldMeshTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WorldMaterial>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some((mesh, collider)) = block_on(poll_once(&mut task.0)) {
            commands
                .spawn(PbrBundle {
                    material: material.0.clone(),
                    mesh: meshes.add(mesh),
                    ..default()
                })
                .insert((RigidBody::Fixed, collider));
            commands.entity(entity).despawn_recursive();
        }
    }
}

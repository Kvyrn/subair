use super::kd_tree::{construct_tree, points_in_range};
use super::marching_cubes_tables::{EDGES, POINT_OFFSETS, TRIANGLE_LISTS};
use super::normals::calculate_normals;
use bevy::render::mesh::Indices;
use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use bevy_rapier3d::prelude::*;
use bracket_noise::prelude::*;
use std::time::Instant;
use tracing::{debug, instrument};

const FLOOR: f32 = 0.0;
const VERTEX_GROUP_MAX_DISTANCE: f32 = 1.0e-7;

#[instrument(skip(offset))]
pub fn generate_world(seed: u64, offset: Vec3, size: usize) -> (Mesh, Collider, Vec3) {
    let start = Instant::now();
    let simple_vertices = marching_cubes(size, size, size, seed, offset);
    debug!(
        num_vertices = simple_vertices.len(),
        "Generated mesh in {:.3}ms",
        start.elapsed().as_secs_f32() * 1000.0
    );
    let (vertices, indices) = deduplicate_vertices(simple_vertices);
    let vertices: Vec<_> = vertices.into_iter().map(|v| v.to_array()).collect();
    let normals = calculate_normals(&vertices, &indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices.clone())));

    let collider_indices = {
        let mut vec = vec![];
        for i in 0..(indices.len() / 3) {
            vec.push([indices[i * 3], indices[i * 3 + 1], indices[i * 3 + 2]]);
        }
        vec
    };
    let collider = Collider::trimesh(
        vertices.into_iter().map(|v| v.into()).collect(),
        collider_indices,
    );
    (mesh, collider, offset)
}

#[instrument(skip(input))]
fn deduplicate_vertices(input: Vec<Vec3>) -> (Vec<Vec3>, Vec<u32>) {
    let start = Instant::now();
    let tree = construct_tree(&input);
    debug!(
        "Constructed tree in {:.3}ms",
        start.elapsed().as_secs_f32() * 1000.0
    );

    let start = Instant::now();
    let mut indices = vec![None; input.len()];
    let mut vertices = vec![];

    for (index, point) in input.iter().enumerate() {
        if indices[index].is_some() {
            continue;
        }
        let vert_index = vertices.len();
        vertices.push(*point);
        points_in_range(
            &tree,
            *point,
            VERTEX_GROUP_MAX_DISTANCE,
            |(_, close_point_index)| {
                indices[close_point_index] = Some(vert_index);
            },
        );
    }
    debug!(
        "Deduplicated vertices in {:.3}ms, removed {:.2}% of vertices",
        start.elapsed().as_secs_f32() * 1000.0,
        (1.0 - vertices.len() as f32 / (indices.len() as f32)) * 100.0
    );
    let indices = indices
        .into_iter()
        .map(|i| i.expect("Point not found (shouldn't happen)") as u32)
        .collect();
    (vertices, indices)
}

#[instrument(skip_all)]
fn marching_cubes(
    width: usize,
    height: usize,
    depth: usize,
    seed: u64,
    noise_offset: Vec3,
) -> Vec<Vec3> {
    let noise = init_noise(seed);
    let mut vertices = vec![];

    for x in 0..(width - 1) {
        for y in 0..(height - 1) {
            for z in 0..(depth - 1) {
                let mut configuration = 0u8;
                let mut values = [0.0; 8];
                for (i, offset) in POINT_OFFSETS.iter().enumerate() {
                    let p = add_points([x, y, z], *offset);
                    let p = point_to_vec3(p);
                    let value = sample_noise(p + noise_offset, &noise);
                    if value > FLOOR {
                        configuration |= 1 << i;
                    }
                    values[i] = value;
                }

                let triangles = TRIANGLE_LISTS[configuration as usize];
                for edge in triangles.iter().flatten().copied() {
                    let [vertex1, vertex2] = EDGES[edge];
                    let point1 = point_to_vec3(add_points([x, y, z], POINT_OFFSETS[vertex1]));
                    let point2 = point_to_vec3(add_points([x, y, z], POINT_OFFSETS[vertex2]));
                    let value1 = values[vertex1];
                    let value2 = values[vertex2];
                    let difference = value2 - value1;
                    let distance_to_ground = FLOOR - value1;
                    let floor_point = point1.lerp(point2, distance_to_ground / difference);
                    vertices.push(floor_point);
                }
            }
        }
    }
    vertices
}

fn point_to_vec3(point: [usize; 3]) -> Vec3 {
    Vec3::new(point[0] as f32, point[1] as f32, point[2] as f32)
}

fn sample_noise(point: Vec3, noise: &FastNoise) -> f32 {
    noise.get_noise3d(point.x, point.y, point.z)
}

fn add_points(p1: [usize; 3], p2: [usize; 3]) -> [usize; 3] {
    [p1[0] + p2[0], p1[1] + p2[1], p1[2] + p2[2]]
}

fn init_noise(seed: u64) -> FastNoise {
    let mut noise = FastNoise::seeded(seed);
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(1);
    noise.set_fractal_gain(0.6);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(0.05);
    noise
}

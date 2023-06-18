use bevy::prelude::*;
use std::time::Instant;
use tracing::{debug, info_span, trace};

pub fn calculate_normals(vertices: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let _e = info_span!(
        "calculate_normals",
        num_vertices = vertices.len(),
        num_indices = indices.len()
    )
    .entered();
    trace!("Generating normals");
    let now = Instant::now();
    let vertices: Vec<Vec3> = vertices
        .iter()
        .map(|v| Vec3::new(v[0], v[1], v[2]))
        .collect();

    let mut vertex_normals = vec![Vec3::default(); vertices.len()];
    for face in 0..(indices.len() / 3) {
        let i0 = indices[face * 3] as usize;
        let i1 = indices[(face * 3) + 1] as usize;
        let i2 = indices[(face * 3) + 2] as usize;
        let p0 = vertices[i0];
        let p1 = vertices[i1];
        let p2 = vertices[i2];

        let u = p1 - p0;
        let v = p2 - p1;

        let normal = Vec3::new(
            (u.y * v.z) - (u.z * v.y),
            (u.z * v.x) - (u.x * v.z),
            (u.x * v.y) - (u.y * v.x),
        );
        vertex_normals[i0] += normal;
        vertex_normals[i1] += normal;
        vertex_normals[i2] += normal;
    }

    let output = vertex_normals.iter().map(|v| v.normalize().into());
    debug!(
        "Generated normals, elapsed: {}ms",
        now.elapsed().as_millis()
    );
    output.collect()
}

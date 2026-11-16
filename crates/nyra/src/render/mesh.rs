use std::f32::consts::{PI, TAU};

use nyra_math::Vec3;

use crate::render::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn cube() -> Self {
        Self::cuboid(Vec3::ONE)
    }

    pub fn cuboid(size: Vec3) -> Self {
        assert!(
            size.x.is_finite()
                && size.x > 0.0
                && size.y.is_finite()
                && size.y > 0.0
                && size.z.is_finite()
                && size.z > 0.0,
            "Cuboid dimensions must be finite and greater than zero"
        );

        let mut vertices = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(36);

        let axes: [Vec3; 3] = [Vec3::X, Vec3::Y, Vec3::Z];
        let face_uvs: [[f32; 2]; 4] = [[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

        let half_size = size / 2.0;

        for normal_axis_index in 0..axes.len() {
            let edge_a_axis_index = (normal_axis_index + 1) % axes.len();
            let edge_b_axis_index = (normal_axis_index + 2) % axes.len();

            let half_edge_a = axes[edge_a_axis_index] * half_size[edge_a_axis_index];
            let half_edge_b = axes[edge_b_axis_index] * half_size[edge_b_axis_index];

            for side_sign in [-1.0, 1.0] {
                let face_normal = axes[normal_axis_index] * side_sign;
                let face_center = half_size[normal_axis_index] * face_normal;
                let oriented_half_edge_b = half_edge_b * side_sign;

                let first_vertex_index = vertices.len() as u32;

                for (position, uv) in [
                    face_center - half_edge_a - oriented_half_edge_b,
                    face_center + half_edge_a - oriented_half_edge_b,
                    face_center + half_edge_a + oriented_half_edge_b,
                    face_center - half_edge_a + oriented_half_edge_b,
                ]
                .iter()
                .zip(face_uvs)
                {
                    vertices.push(Vertex::new(position.to_array(), face_normal.to_array(), uv));
                }

                indices.extend_from_slice(&[
                    first_vertex_index,
                    first_vertex_index + 1,
                    first_vertex_index + 2,
                    first_vertex_index,
                    first_vertex_index + 2,
                    first_vertex_index + 3,
                ]);
            }
        }

        Self { vertices, indices }
    }

    pub fn sphere() -> Self {
        Self::uv_sphere(0.5, 32, 16)
    }

    pub fn uv_sphere(radius: f32, radial_segments: u32, height_segments: u32) -> Self {
        assert!(
            radius.is_finite() && radius > 0.0,
            "Sphere radius must be finite and greater than zero"
        );
        assert!(
            radial_segments >= 3,
            "Sphere radial segments must be at least 3"
        );
        assert!(
            height_segments >= 2,
            "Sphere height segments must be at least 2"
        );

        let mut vertices =
            Vec::with_capacity(((radial_segments + 1) * (height_segments + 1)) as usize);
        let mut indices =
            Vec::with_capacity((6 * radial_segments * (height_segments - 1)) as usize);

        for i in 0..=height_segments {
            let v = i as f32 / height_segments as f32;
            let theta = v * PI;

            let (sin_theta, cos_theta) = if i == 0 {
                (0.0, 1.0)
            } else if i == height_segments {
                (0.0, -1.0)
            } else {
                theta.sin_cos()
            };

            for j in 0..=radial_segments {
                let u = j as f32 / radial_segments as f32;
                let phi = u * TAU;

                let (sin_phi, cos_phi) = if j == radial_segments {
                    (0.0, 1.0)
                } else {
                    phi.sin_cos()
                };

                let x = radius * sin_theta * cos_phi;
                let y = radius * cos_theta;
                let z = radius * sin_theta * sin_phi;

                let position = Vec3::new(x, y, z);
                let normal = position / radius;

                vertices.push(Vertex::new(position.to_array(), normal.to_array(), [u, v]));
            }
        }

        let row_size = radial_segments + 1;

        for i in 0..height_segments {
            for j in 0..radial_segments {
                let a = i * row_size + j;
                let b = a + 1;
                let c = a + row_size;
                let d = c + 1;

                if i != 0 {
                    indices.extend_from_slice(&[a, b, c]);
                }

                if i + 1 != height_segments {
                    indices.extend_from_slice(&[b, d, c]);
                }
            }
        }

        Self { vertices, indices }
    }
}

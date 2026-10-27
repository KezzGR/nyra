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
        let uves: [[f32; 2]; 4] = [[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

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
                .zip(uves)
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
}

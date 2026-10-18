use nyra_math::Vec3;

use crate::render::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn cube() -> Self {
        Mesh::cuboid(Vec3::ONE)
    }

    pub fn cuboid(size: Vec3) -> Self {
        let h = size / 2.0;
        let axes = [Vec3::X, Vec3::Y, Vec3::Z];

        let mut vertices = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(36);

        for i in 0..3 {
            let a = axes[(i + 1) % 3] * h[(i + 1) % 3];
            let b = axes[(i + 2) % 3] * h[(i + 2) % 3];

            for sign in [-1.0, 1.0] {
                let n = axes[i] * sign;
                let c = h[i] * n;
                let b = b * sign;

                let base = vertices.len() as u16;

                for position in [c - a - b, c + a - b, c + a + b, c - a + b] {
                    vertices.push(Vertex::new(position.to_array(), n.to_array()));
                }

                indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            }
        }

        Self { vertices, indices }
    }
}

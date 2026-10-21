use crate::Transform;

pub struct Thing {
    pub transform: Transform,
    pub mesh_id: usize,
}

impl Thing {
    pub fn new(transform: Transform, mesh_id: usize) -> Self {
        Self { transform, mesh_id }
    }
}

use nyra_math::Transform;

use crate::{EntityID, MaterialID, MeshID};

pub struct Entity {
    pub id: EntityID,
    pub transform: Transform,
    pub mesh_id: MeshID,
    pub visible: bool,
    pub material: Option<MaterialID>,
    pub parent: Option<EntityID>,
}

impl Entity {
    pub fn new(transform: Transform, mesh_id: MeshID) -> Self {
        Self {
            transform,
            mesh_id,
            id: EntityID(0),
            visible: true,
            material: None,
            parent: None,
        }
    }

    pub fn with_material(mut self, material: MaterialID) -> Self {
        self.material = Some(material);

        self
    }
}

use nyra_math::Transform;

use crate::{EntityID, MaterialID, MeshID, Renderable};

pub struct Entity {
    pub id: EntityID,
    pub transform: Transform,
    pub renderable: Option<Renderable>,
    pub parent: Option<EntityID>,
}

impl Entity {
    pub fn empty(transform: Transform) -> Self {
        Self {
            id: EntityID(0),
            transform,
            renderable: None,
            parent: None,
        }
    }

    pub fn from_mesh(transform: Transform, mesh_id: MeshID) -> Self {
        Self {
            id: EntityID(0),
            transform,
            renderable: Some(Renderable::new(mesh_id)),
            parent: None,
        }
    }

    pub fn with_material(mut self, material_id: MaterialID) -> Self {
        self.renderable
            .as_mut()
            .expect("Cannot assign material to an empty entity")
            .set_material(material_id);

        self
    }
}

use crate::{MaterialID, ThingID, Transform};

pub struct Thing {
    pub transform: Transform,
    pub mesh_id: usize,
    pub id: ThingID,
    pub visible: bool,
    pub material: Option<MaterialID>,
}

impl Thing {
    pub fn new(transform: Transform, mesh_id: usize) -> Self {
        Self {
            transform,
            mesh_id,
            id: ThingID(0),
            visible: true,
            material: None,
        }
    }

    pub fn with_material(mut self, material: MaterialID) -> Self {
        self.material = Some(material);

        self
    }
}

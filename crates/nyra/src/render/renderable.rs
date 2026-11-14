use crate::{MaterialID, MeshID};

pub struct Renderable {
    pub visible: bool,
    pub mesh_id: MeshID,
    pub material_id: Option<MaterialID>,
}

impl Renderable {
    pub fn new(mesh_id: MeshID) -> Self {
        Self {
            visible: true,
            mesh_id,
            material_id: None,
        }
    }

    pub fn set_material(&mut self, material_id: MaterialID) {
        self.material_id = Some(material_id);
    }
}

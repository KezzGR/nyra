use std::collections::HashMap;

use crate::{Material, MaterialID, Mesh, MeshID, Texture, TextureID};

#[derive(Default)]
pub struct Assets {
    pub mesh_storage: HashMap<MeshID, Mesh>,
    pub material_storage: HashMap<MaterialID, Material>,
    pub texture_storage: HashMap<TextureID, Texture>,

    next_mesh_id: u64,
    next_material_id: u64,
    next_texture_id: u64,
}

impl Assets {
    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshID {
        let id = MeshID(self.next_mesh_id);

        self.next_mesh_id += 1;

        self.mesh_storage.insert(id, mesh);

        id
    }

    pub fn add_material(&mut self, material: Material) -> MaterialID {
        let id = MaterialID(self.next_material_id);

        self.next_material_id += 1;

        self.material_storage.insert(id, material);

        id
    }

    pub fn add_texture(&mut self, texture: Texture) -> TextureID {
        let id = TextureID(self.next_texture_id);

        self.next_texture_id += 1;

        self.texture_storage.insert(id, texture);

        id
    }
}

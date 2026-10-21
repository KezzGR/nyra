pub mod thing;
pub mod transform;

pub use thing::Thing;

use crate::Mesh;

#[derive(Default)]
pub struct World {
    pub things: Vec<Thing>,
    pub mesh_storage: Vec<Mesh>,
}

impl World {
    pub fn spawn(&mut self, thing: Thing) {
        self.things.push(thing);
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        self.mesh_storage.push(mesh);

        self.mesh_storage.len() as usize - 1
    }
}

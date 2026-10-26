pub mod thing;
pub mod thing_id;
pub mod transform;

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

pub use thing::Thing;
pub use thing_id::ThingID;

use crate::{Material, MaterialID, Mesh};

#[derive(Default)]
pub struct World {
    pub things: Vec<Thing>,
    pub mesh_storage: Vec<Mesh>,
    pub material_storage: HashMap<MaterialID, Material>,
    next_id: u64,
    next_material_id: u64,
}

impl World {
    pub fn spawn(&mut self, mut thing: Thing) -> ThingID {
        let id = ThingID(self.next_id);

        self.next_id += 1;
        thing.id = id;

        self.things.push(thing);

        id
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        self.mesh_storage.push(mesh);

        self.mesh_storage.len() - 1
    }

    pub fn add_material(&mut self, material: Material) -> MaterialID {
        let id = MaterialID(self.next_material_id);

        self.next_material_id += 1;

        self.material_storage.insert(id, material);

        id
    }

    pub fn despawn(&mut self, thing_id: ThingID) {
        if let Some(index) = self.things.iter().position(|t| t.id == thing_id) {
            self.things.remove(index);
        }
    }

    pub fn get(&self, id: ThingID) -> Option<&Thing> {
        self.things.iter().find(|thing| thing.id == id)
    }

    pub fn get_mut(&mut self, id: ThingID) -> Option<&mut Thing> {
        self.things.iter_mut().find(|thing| thing.id == id)
    }
}

impl Index<ThingID> for World {
    type Output = Thing;

    fn index(&self, index: ThingID) -> &Self::Output {
        self.get(index).expect("Thing not found")
    }
}

impl IndexMut<ThingID> for World {
    fn index_mut(&mut self, index: ThingID) -> &mut Self::Output {
        self.get_mut(index).expect("Thing not found")
    }
}

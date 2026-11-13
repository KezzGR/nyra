mod directional_light;
mod entity;
mod entity_id;
mod environment;

use std::ops::{Index, IndexMut};

pub use directional_light::DirectionalLight;
pub use entity::Entity;
pub use entity_id::EntityID;
pub use environment::Environment;
use nyra_math::Mat4;

#[derive(Default)]
pub struct World {
    pub entities: Vec<Entity>,
    pub environment: Environment,
    next_id: u64,
}

impl World {
    pub fn spawn(&mut self, mut entity: Entity) -> EntityID {
        let id = EntityID(self.next_id);

        self.next_id += 1;
        entity.id = id;

        self.entities.push(entity);

        id
    }

    pub fn despawn(&mut self, entity_id: EntityID) {
        if let Some(index) = self
            .entities
            .iter()
            .position(|entity| entity.id == entity_id)
        {
            self.entities.remove(index);
        }
    }

    pub fn get(&self, id: EntityID) -> Option<&Entity> {
        self.entities.iter().find(|entity| entity.id == id)
    }

    pub fn get_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|entity| entity.id == id)
    }

    pub fn set_parent(&mut self, child_id: EntityID, parent_id: EntityID) {
        if self.get(child_id).is_none() || self.get(parent_id).is_none() {
            panic!("AAAAAAAAAAAAA PIZDETS")
        }

        let mut current = Some(parent_id);

        while let Some(id) = current {
            if id == child_id {
                panic!("AAAAAAAAAAAAAAAAAAAA")
            }

            current = self.get_mut(id).unwrap().parent;
        }

        self.get_mut(child_id).unwrap().parent = Some(parent_id);
    }

    pub fn model_matrix(&self, id: EntityID) -> Mat4 {
        let mut current = Some(id);
        let mut matrix = Mat4::IDENTITY;

        while let Some(id) = current {
            let entity = self.get(id).unwrap();

            matrix = entity.transform.to_matrix() * matrix;

            current = entity.parent;
        }

        matrix
    }
}

impl Index<EntityID> for World {
    type Output = Entity;

    fn index(&self, index: EntityID) -> &Self::Output {
        self.get(index).expect("Entity not found")
    }
}

impl IndexMut<EntityID> for World {
    fn index_mut(&mut self, index: EntityID) -> &mut Self::Output {
        self.get_mut(index).expect("Entity not found")
    }
}

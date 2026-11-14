mod directional_light;
mod entity;
mod entity_id;
mod environment;

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

pub use directional_light::DirectionalLight;
pub use entity::Entity;
pub use entity_id::EntityID;
pub use environment::Environment;
use nyra_math::Mat4;

#[derive(Default)]
pub struct World {
    pub entities: HashMap<EntityID, Entity>,
    pub environment: Environment,
    next_entity_id: u64,
}

impl World {
    pub fn spawn(&mut self, entity: Entity) -> EntityID {
        let id = EntityID(self.next_entity_id);

        self.next_entity_id += 1;

        self.entities.insert(id, entity);

        id
    }

    pub fn despawn(&mut self, entity_id: EntityID) {
        if !self.entities.contains_key(&entity_id) {
            return;
        }

        let mut stack = vec![entity_id];

        while let Some(id) = stack.pop() {
            let children = self
                .entities
                .iter()
                .filter(|(_, entity)| entity.parent == Some(id))
                .map(|(id, _)| *id);

            stack.extend(children);

            self.entities.remove(&id);
        }
    }

    pub fn get(&self, id: EntityID) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
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

            current = self.get(id).unwrap().parent;
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

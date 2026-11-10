mod directional_light;
mod entity;
mod entity_id;
mod environment;

use std::ops::{Index, IndexMut};

pub use directional_light::DirectionalLight;
pub use entity::Entity;
pub use entity_id::EntityID;
pub use environment::Environment;

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

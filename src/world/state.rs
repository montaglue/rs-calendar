use super::{place::Place, item::Storages, action::Action};


#[derive(Debug, Default)]
pub struct WorldState {
    action: Action,
    place: Place,
    storages: Storages,
}

impl WorldState {
    pub fn new(action: Action, place: Place) -> WorldState {
        WorldState {
            action,
            place,
            storages: Default::default(),
        }
    }
}

use crate::{combinatorics_engine::bounds::{TimeBound, StorageBound, ItemBound, PlaceBound}, world::{action::Action, state::WorldState, item::Storages}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    NotStarted,
    Planed,
    InProgres,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub(crate) time: Vec<TimeBound>,
    pub(crate) storage: Vec<StorageBound>,
    pub(crate) item: Vec<ItemBound>,
    pub(crate) place: Vec<PlaceBound>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub(crate) name: String,
    pub(crate) importance: usize,
    pub(crate) description: String,
    pub(crate) stage: Stage,
    pub(crate) bounds: Bounds,
    pub(crate) action: Action,
}

impl Task {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn state(&self) -> WorldState {
        let avalible_places = PlaceBound::intersect(&self.bounds.place).place();
        WorldState::new(self.action.clone(), avalible_places)
    }
}

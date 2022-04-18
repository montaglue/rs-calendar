use std::collections::{HashMap, HashSet};

use crate::world::{Time, place::Place, item::{Storage, Item}};

#[derive(Debug, Clone)]
pub enum TimeBound {
    Begin(Time),
    BeginEnd(Time, Time),
    Deadline(Time),
    Day(Time),
    Duration(Time),
}

#[derive(Debug, Clone)]
pub enum StorageBound {
    Contain(HashMap<Storage, Item>),
}

#[derive(Debug, Clone)]
pub enum ItemBound {
    Dependency(Item, Item),
}

#[derive(Debug, Clone)]
pub enum PlaceBound {
    OneOf(Vec<Place>),
}

impl PlaceBound {
    pub fn intersect(bounds: Vec<PlaceBound>) -> PlaceBound {
        todo!()
    }

    pub fn places(self) -> Vec<Place> {
        match self {
            PlaceBound::OneOf(places) => places,
        }
    }

    pub fn place(self) -> Place {
        match self {
            PlaceBound::OneOf(mut ps) => ps.pop().unwrap_or_default(),
        }
    }
}

pub enum TaskBound {} // other tasks, WIP

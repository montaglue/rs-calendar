use std::{collections::HashMap};

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Clothes,
    Other,
}

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    item_type : ItemType,
    description: String,
}

#[derive(Debug, Clone)]
pub struct Storage {
    name: String,
    containments: Vec<Item>,
}

#[derive(Debug, Default, Clone)]
pub struct Storages(HashMap<String, Storage>);
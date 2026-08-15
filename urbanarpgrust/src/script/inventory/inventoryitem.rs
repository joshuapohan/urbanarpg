use std::collections::HashMap;

use godot::prelude::*;
use godot::{classes::{FileAccess, file_access::ModeFlags}};
use serde::Deserialize;


#[derive(Clone, Deserialize)]
pub struct InventoryItemJSON {
    id: String,
    count: i32,
}


#[derive(Clone)]
pub struct InventoryItem{
    id: GString,
    pub count: i32,
}

impl From<InventoryItemJSON> for InventoryItem {
    fn from(value: InventoryItemJSON) -> Self {
        Self {
            id: value.id.to_gstring(),
            count: value.count
        }
    }
}

pub fn load_items() -> HashMap<GString, InventoryItem> {
    let mut items = HashMap::<GString, InventoryItem>::new();
    let mut file = FileAccess::open("res://data/inventory.json", ModeFlags::READ).unwrap();
    let content = file.get_as_text();
    let parsed_items_json: HashMap<String, InventoryItemJSON> = serde_json::from_str(&content.to_string()).unwrap();
    for (key, node) in parsed_items_json {
        items.insert(key.to_gstring(), node.into());
    }
    items
}
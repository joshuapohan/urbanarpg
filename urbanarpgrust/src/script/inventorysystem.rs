use crate::log_error;
use crate::log_info;
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(singleton)]
pub struct InventorySystem {
    base: Base<Object>,
    inventory_items: HashMap<GString, InventoryItem>,
}

#[godot_api]
impl InventorySystem {}

#[godot_api]
impl IObject for InventorySystem {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            inventory_items: HashMap::<GString, InventoryItem>::new(),
        }
    }
}

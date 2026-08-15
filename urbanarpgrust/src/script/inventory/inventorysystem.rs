use godot::prelude::*;
use std::collections::HashMap;
use crate::log_info;
use crate::log_error;
use crate::script::inventory::inventoryitem::InventoryItem;
use crate::script::inventory::inventoryitem::load_items;

#[derive(GodotClass)]
#[class(singleton)]
pub struct InventorySystem {

    base: Base<Object>,
    inventory: HashMap<GString, InventoryItem>,
}


#[godot_api]
impl InventorySystem {
    #[func]
    pub fn add_item(&mut self, id: GString, count: i32){
        if let Some(item)  = self.inventory.get_mut(&id) {
            item.count += count;
            log_info!("Item added {} : {} total: {}", id, count, item.count);
        } else {
            log_error!("Item id not found {}", id);
        }
    }
}


#[godot_api]
impl IObject for InventorySystem {
    fn init(base: Base<Object>) -> Self {
        Self{
            base,
            inventory: load_items(),
        }
    }
}

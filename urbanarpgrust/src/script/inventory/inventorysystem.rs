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


    #[func]
    pub fn check_if_item_exist(&self, id: GString) -> bool{
        if let Some(item)  = self.inventory.get(&id) {
            return item.count > 0;
        } else {
            log_error!("Item id not found {}", id);
            return false;
        }        
    }

    #[func]
    pub fn  take_item(&mut self, id: GString,  count: i32) -> i32 {
        if let Some(item) = self.inventory.get_mut(&id){
            if count > item.count {
                log_error!("Item count insufficient {} {} {}", id, item.count, count);
                return -1;
            }
            item.count = item.count - 1;
            return item.count;
        } else {
            log_error!("Item not found {}", id);
            return -1;
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

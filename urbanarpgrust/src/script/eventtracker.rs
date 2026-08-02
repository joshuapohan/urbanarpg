use std::collections::HashMap;

use godot::prelude::*;
use crate::script::eventdetail::{self, EventDetail};


#[derive(GodotClass)]
#[class(singleton)]
pub struct EventTracker {

    base: Base<Object>,
    events:  HashMap<GString, EventDetail>

}

#[godot_api]
impl EventTracker{

    pub fn check_if_event(&self, event_id : &str) -> i32 {
        match self.events.get(&event_id.to_gstring()){
            Some(event) => {
                if event.is_triggered {
                    event.trigger_count
                } else {
                    0
                }
            }, 
            None => {
                godot_error!("[check_if_event] Event id not found: {}", event_id);
                0
            }
        }
        
    }

    #[func]
    pub fn trigger_event(&mut self, event_id: GString){
        match self.events.get_mut(&event_id){
            Some(event) => {
                if event.is_triggered && event.is_one_time {
                    return
                } else {
                    event.is_triggered = true;
                    event.trigger_count = event.trigger_count + 1;
                    godot_print!("Event triggered : {} : {}", event.id, event.trigger_count);
                }
            },
            None => {
                godot_error!("[trigger_event] Event id not found: {}", event_id);                
            }
        }
    }

}

#[godot_api]
impl IObject for EventTracker {
    fn init(base: Base<Object>) -> Self {
        let events = eventdetail::load_events();
        Self {
            base,
            events
        }
    }
}
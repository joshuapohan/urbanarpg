use std::collections::HashMap;
use godot::{classes::{FileAccess, file_access::ModeFlags}, prelude::*};
use serde::Deserialize;


#[derive(Clone)]
pub struct EventDetail{
    pub id: GString,
    pub is_one_time: bool,
    pub trigger_count: i32,
    pub is_triggered: bool,
}


#[derive(Clone, Deserialize)]
pub struct EventDetailJSON{
    id: String,
    is_one_time: bool,
}

impl From<EventDetailJSON> for EventDetail {
    fn from(value: EventDetailJSON) -> Self {
        EventDetail { 
            id: value.id.to_gstring(), 
            is_one_time: value.is_one_time, 
            trigger_count: 0, 
            is_triggered: false 
        }
    }
}

pub fn load_events() -> HashMap<GString, EventDetail>{
    let mut events = HashMap::<GString, EventDetail>::new();
    let mut file = FileAccess::open("res://data/events.json", ModeFlags::READ).unwrap();
    let content = file.get_as_text();
    let parsed_events_json: HashMap<String, EventDetailJSON> = serde_json::from_str(&content.to_string()).unwrap();
    for (key, eventdetail) in parsed_events_json {
        events.insert(key.to_gstring(), eventdetail.into());
    }
    events
}
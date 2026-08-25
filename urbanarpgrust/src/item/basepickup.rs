use godot::prelude::*;
use godot::classes::AudioStreamPlayer2D;
use godot::classes::Area2D;

use crate::script::dialogue::dialoguesystem::DialogueSystem;
use crate::script::event::eventtracker::EventTracker;
use crate::script::gamestate::GameState;

pub struct BasePickup {    
    pub pickup_audio: Option<Gd<AudioStreamPlayer2D>>,
    pub name: String,
}

impl BasePickup{

    pub fn on_pickup(&mut self, mut base: Gd<Area2D>){
        base.hide();
        base.set_deferred("monitoring", &false.to_variant());
        base.set_deferred("monitorable", &false.to_variant());
        self.pickup_audio.as_mut().unwrap().play();

        let pickup_event_dialogue_id = format!("first_pickup_{}", self.name);
        
        let is_already_picked_up = EventTracker::singleton().bind().check_if_event(&pickup_event_dialogue_id) > 0;
        if is_already_picked_up {
            // already triggered first pickup
        } else {
            GameState::singleton().bind_mut().set_game_context_dialogue();
            GameState::singleton().bind_mut().pause_gameplay();
            EventTracker::singleton().bind_mut().trigger_event(pickup_event_dialogue_id.to_gstring());
            DialogueSystem::singleton().bind_mut().start_dialogue_by_id(pickup_event_dialogue_id.to_gstring());
        }
    }
}
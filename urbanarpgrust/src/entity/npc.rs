use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Area2D, AudioStreamPlayer2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Timer, Tween};

use crate::entity::traits::Interactable;
use crate::script::dialoguesystem::DialogueSystem;
use crate::script::eventtracker;
use crate::script::gamestate::GameState;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct NPC {
    base: Base<CharacterBody2D>,
    
    #[export]
    interaction_id: GString,
    #[export]
    interaction_type: GString,

    #[export]
    event_check_ids: Array<GString>,
    #[export]
    event_check_pass_interaction_id: GString, // will run this if all event check id pass
}

#[godot_api]
impl ICharacterBody2D for NPC {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self{
            base: base,
            interaction_id: "0".to_gstring(),
            interaction_type: "npc_0".into(),
            event_check_ids: Array::new(),
            event_check_pass_interaction_id: "0".to_gstring(),
        }
    }

    fn ready(&mut self){
        godot_print!("interaction_id: {}", self.interaction_id);
    }
  
}

#[godot_api]
impl NPC {
    #[signal]
    fn s_npc_interacted(interaction_type: GString, interaction_id: GString);

    #[func]
    fn interact(&mut self) {
        godot_print!("Interaction triggered 1");
        Interactable::interact(self); // delegate to trait impl
    }  
}


impl Interactable for NPC {
    fn interact(&mut self) {
        godot_print!("[interact] Interaction triggered 2");
        
        let interaction_id = if self.event_check_ids.len() > 0 && self.event_check_pass_interaction_id.len() > 0 {
            let mut id= self.event_check_pass_interaction_id.clone();
            'eventcheck: for event_check in self.event_check_ids.iter_shared() {
                if eventtracker::EventTracker::singleton().bind().check_if_event(&event_check.to_string()) <= 0 {
                    id = self.interaction_id.clone();
                    godot_print!("[interact] Interaction triggered , event not passed: {}", event_check);                    
                    break 'eventcheck;
                }                            
            }
            id
        } else {
            godot_print!("[interact] Interaction triggered , no event check");                    
            self.interaction_id.clone()
        };
        
        GameState::singleton().bind_mut().set_game_context_dialogue();
        GameState::singleton().bind_mut().pause_gameplay();        
        DialogueSystem::singleton().bind_mut().start_dialogue_by_id(interaction_id);
    }
}
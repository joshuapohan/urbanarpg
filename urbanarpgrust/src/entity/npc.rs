use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Area2D, AudioStreamPlayer2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Timer, Tween};

use crate::entity::traits::Interactable;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct NPC {
    base: Base<CharacterBody2D>,
    
    #[export]
    interaction_id : GString,
    #[export]
    interaction_type: GString,    
}

#[godot_api]
impl ICharacterBody2D for NPC {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self{
            base: base,
            interaction_id: "123".into(),
            interaction_type: "npc_0".into(),
        }
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
        godot_print!("Interaction triggered 2");
        let interaction_id = self.interaction_id.clone();
        let interaction_type = self.interaction_type.clone();
        self.signals().s_npc_interacted().emit( &interaction_type, &interaction_id);
    }
}
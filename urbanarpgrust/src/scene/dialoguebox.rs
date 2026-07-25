use godot::prelude::*;
use godot::classes::{CanvasLayer, ICanvasLayer, RichTextLabel, Timer};

use crate::script::gamestate;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct DialogueBox {
    #[base]
    base: Base<CanvasLayer>,

    name_label: Option<Gd<RichTextLabel>>,
    text_label: Option<Gd<RichTextLabel>>,
    effect_timer: Option<Gd<Timer>>,

    line_len: i32,
}

#[godot_api]
impl DialogueBox {
    #[func]
    fn on_dialogue_start(&mut self, line: GString, speaker: GString, avatar_id: GString){
        godot_print!("Dialoguebox started {} {}", speaker, line);
        gamestate::GameState::singleton().bind_mut().set_dialogue_state_start();        
        self.base_mut().show();
        self.name_label.as_mut().unwrap().set_text(&speaker);
        self.text_label.as_mut().unwrap().set_text(&line);
        self.text_label.as_mut().unwrap().set_visible_characters(0);
        self.line_len = line.len() as i32;
        self.effect_timer.as_mut().unwrap().start();
    }

    #[func]
    fn on_dialogue_end(&mut self){
        self.base_mut().hide();
        godot_print!("Dialoguebox ended");
        gamestate::GameState::singleton().bind_mut().set_game_context_level(0);
        gamestate::GameState::singleton().bind_mut().resume_gameplay();
    }
    
    #[func]
    fn on_skip_typewriter(&mut self){
        self.text_label.as_mut().unwrap().set_visible_characters(-1);
        godot_print!("Showing all characters");
        self.effect_timer.as_mut().unwrap().stop();
        gamestate::GameState::singleton().bind_mut().set_dialogue_state_end();        
    }

    #[func]
    fn on_effect_timer_timeout(&mut self){
        let current = self.text_label.as_ref().unwrap().get_visible_characters();
        self.text_label.as_mut().unwrap().set_visible_characters(current + 1);
        godot_print!("Showing more characters");
        if current + 1 >= self.line_len {
            self.effect_timer.as_mut().unwrap().stop();
            gamestate::GameState::singleton().bind_mut().set_dialogue_state_end();
        }
    }
    
    #[func]
    fn on_choice_shown(&mut self, choices: Vec<GString>){

    }

   #[func]
    fn s_choice_highlighted(&mut self, choices: Vec<GString>, index: i32){
        
    }    

}

#[godot_api]
impl ICanvasLayer for DialogueBox{
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base,
            name_label: None,
            text_label: None,
            effect_timer: None,
            line_len: 0,
        }
    }

    
    fn ready(&mut self){
        self.name_label = self.base().get_node_as::<RichTextLabel>("MarginContainer/VBoxContainer/Name").into(); 
        self.text_label = self.base().get_node_as::<RichTextLabel>("MarginContainer/VBoxContainer/Text").into(); 
        self.effect_timer = self.base().get_node_as::<Timer>("EffectTimer").into();

        let on_effect_timer_timeout_callback = Callable::from_object_method(&self.base(), "on_effect_timer_timeout");
        self.effect_timer.as_mut().unwrap().connect("timeout", &on_effect_timer_timeout_callback);
    }
}
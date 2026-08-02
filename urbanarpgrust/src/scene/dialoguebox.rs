use godot::prelude::*;
use godot::classes::{CanvasLayer, ICanvasLayer, Label, PanelContainer, RichTextLabel, StyleBox, Timer, VBoxContainer};
use serde_json::map::Iter;

use crate::script::gamestate;
use crate::log_info;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct DialogueBox {
    #[base]
    base: Base<CanvasLayer>,

    #[export]
    normal_style: Option<Gd<StyleBox>>,
    #[export]
    highlighted_style: Option<Gd<StyleBox>>,

    name_label: Option<Gd<RichTextLabel>>,
    text_label: Option<Gd<RichTextLabel>>,
    effect_timer: Option<Gd<Timer>>,
    choice_container: Option<Gd<VBoxContainer>>,
    choice_scene: Option<Gd<PackedScene>>,

    line_len: i32,
}

#[godot_api]
impl DialogueBox {
    #[func]
    fn on_dialogue_start(&mut self, line: GString, speaker: GString, avatar_id: GString){
        log_info!("Dialoguebox started {} {}", speaker, line);

        self.choice_container.as_mut().unwrap().hide();

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
        log_info!("Dialoguebox ended");
        gamestate::GameState::singleton().bind_mut().set_game_context_level(0);
        gamestate::GameState::singleton().bind_mut().resume_gameplay();
    }
    
    #[func]
    fn on_skip_typewriter(&mut self){
        self.text_label.as_mut().unwrap().set_visible_characters(-1);
        log_info!("Showing all characters");
        self.effect_timer.as_mut().unwrap().stop();
        gamestate::GameState::singleton().bind_mut().set_dialogue_state_end();        
    }

    #[func]
    fn on_effect_timer_timeout(&mut self){
        let current = self.text_label.as_ref().unwrap().get_visible_characters();
        self.text_label.as_mut().unwrap().set_visible_characters(current + 1);
        log_info!("Showing more characters");
        if current + 1 >= self.line_len {
            self.effect_timer.as_mut().unwrap().stop();
            gamestate::GameState::singleton().bind_mut().set_dialogue_state_end();
        }
    }
    
    #[func]
    fn on_choice_shown(&mut self, choices: Vec<GString>){
        self.name_label.as_mut().unwrap().set_text("");
        self.text_label.as_mut().unwrap().set_text("");

        log_info!("Dialogue box choice show");

        self.choice_container.as_mut().unwrap().show();

        let container = self.choice_container.as_mut().unwrap();
        for mut child in container.get_children().iter_shared(){
            child.queue_free();
        }


        /* 
        for (current_index, choice) in choices.iter().enumerate() {
            let mut choice_box = self.choice_scene.as_ref().unwrap().instantiate_as::<PanelContainer>();
            choice_box.get_node_as::<Label>("MarginContainer/HBoxContainer/ChoiceText").set_text(choice);
            if current_index == 0 {
                choice_box.add_theme_stylebox_override("panel", self.highlighted_style.as_ref().unwrap());
            }
            container.add_child(&choice_box);
        }
        */        
        self.base_mut().call_deferred("deferred_show_choices", &[choices.to_variant()]);

    }

    #[func]
    fn deferred_show_choices(&mut self, choices: Vec<GString>){

        let container = self.choice_container.as_mut().unwrap();

        for (current_index, choice) in choices.iter().enumerate() {
            let mut choice_box = self.choice_scene.as_ref().unwrap().instantiate_as::<PanelContainer>();
            choice_box.get_node_as::<Label>("MarginContainer/HBoxContainer/ChoiceText").set_text(choice);
            if current_index == 0 {
                choice_box.add_theme_stylebox_override("panel", self.highlighted_style.as_ref().unwrap());
            }
            container.add_child(&choice_box);
        }
    }

    #[func]
    fn on_choice_highlighted(&mut self, index: i32){
        self.name_label.as_mut().unwrap().set_text("");
        self.text_label.as_mut().unwrap().set_text("");

        log_info!("Dialogue box choice highlight");        

        self.choice_container.as_mut().unwrap().show();        

        let container = self.choice_container.as_mut().unwrap();
        for  (current_index, mut child) in container.get_children().iter_shared().enumerate() {
            let mut panel_container = child.cast::<PanelContainer>();
            if current_index as i32 == index {
                panel_container.add_theme_stylebox_override("panel", self.highlighted_style.as_ref().unwrap());
            } else {
                panel_container.add_theme_stylebox_override("panel", self.normal_style.as_ref().unwrap());
            }
        }
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
            choice_container: None,
            choice_scene: None,
            normal_style: None, 
            highlighted_style: None,
        }
    }

    
    fn ready(&mut self){
        self.name_label = self.base().get_node_as::<RichTextLabel>("MarginContainer/VBoxContainer/Name").into(); 
        self.text_label = self.base().get_node_as::<RichTextLabel>("MarginContainer/VBoxContainer/Text").into(); 
        self.effect_timer = self.base().get_node_as::<Timer>("EffectTimer").into();
        self.choice_container = self.base().get_node_as::<VBoxContainer>("MarginContainer/ScrollContainer/ChoiceContainer").into(); 
        self.choice_scene = Some(load::<PackedScene>("res://scenes/ui/choicebox.tscn"));

        let on_effect_timer_timeout_callback = Callable::from_object_method(&self.base(), "on_effect_timer_timeout");
        self.effect_timer.as_mut().unwrap().connect("timeout", &on_effect_timer_timeout_callback);
    }
}
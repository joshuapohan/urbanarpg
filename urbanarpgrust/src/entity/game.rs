use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};
use godot::classes::{INode2D, InputEvent, Node2D};
use godot::prelude::*;
use crate::entity::adventurer::Adventurer;
use crate::scene::dialoguebox::DialogueBox;
use crate::scene::hud::HUD;
use crate::scene::mainmenu::MainMenu;
use crate::script::gamestate::{self, GameState};
use crate::script::dialoguesystem::DialogueSystem;
use crate::template::levelroot::LevelRoot;
use crate::template::portal::Portal;


#[derive(GodotClass)]
#[class(base=Node2D)]
struct MainNode {

    base: Base<Node2D>,

    current_level: Option<Gd<LevelRoot>>,
    player: Option<Gd<Adventurer>>,
    hud: Option<Gd<HUD>>,
    main_menu: Option<Gd<MainMenu>>,
    dialogue_box: Option<Gd<DialogueBox>>,
}

#[godot_api]
impl MainNode{
    #[func]
    fn on_hud_fade_out_complete(&mut self){
        godot_print!("HUD Fade out Timeout");
        self.base_mut().call_deferred("load_level", &[1.to_variant()]);
        self.player.as_mut().unwrap().bind_mut().reset();
        self.hud.as_mut().unwrap().bind_mut().fade(0.0);          
    }

    #[func]
    fn on_hud_fade_in_complete(&mut self){
        godot_print!("HUD Fade in Timeout");
    }


    #[func]
    fn on_player_death_timeout(&mut self){
        godot_print!("Player Death Signal Timeout");        
        //self.player.as_mut().unwrap().bind_mut().reset();
        self.hud.as_mut().unwrap().bind_mut().fade(1.0);  
    }

    #[func]
    fn on_player_death_timeout_2(&mut self){
        godot_print!("Player Death Signal Timeout");
        self.base_mut().call_deferred("load_level", &[1.to_variant()]);
        self.player.as_mut().unwrap().bind_mut().reset();  
    }    
      
    #[func]
    fn on_player_death(&mut self){
        godot_print!("Player Death Signal Received");
        let mut tree = self.base().get_tree();
        let mut timer = tree.create_timer(1.0);
        let callable = Callable::from_object_method(&self.base(), "on_player_death_timeout");
        timer.connect("timeout", &callable);
        /* 
        self.base_mut().call_deferred("load_level", &[1.to_variant()]);
        self.player.as_mut().unwrap().bind_mut().reset();        
        */        
    }    


    #[func]
    fn on_portal_entered(&mut self, body: Gd<Node2D>){
        if body.get_name().contains("Adventurer"){
            godot_print!("Portal Entered");
            // get target map id
            let portal: Option<Gd<Portal>> = self.current_level.as_ref().unwrap().get_node_as::<Portal>("Portal").into();        
            let target_level_id = portal.as_ref().unwrap().bind().get_portal_target_map_id();
            godot_print!("Loading {}", target_level_id);
            self.base_mut().call_deferred("load_level", &[target_level_id.to_variant()]);
            
            //self.load_level();
        }
    }

    #[func]
    fn on_interaction_callback(&mut self, interaction_type: GString, interaction_id: GString){
        godot_print!("Interaction triggered with type : {}, id : {}", interaction_type, interaction_id);
        match interaction_type.to_string().as_str() {
            "npc_0" => {
                DialogueSystem::singleton().bind_mut().start_dialogue_by_id(interaction_id);
                GameState::singleton().bind_mut().set_game_context_dialogue();
                GameState::singleton().bind_mut().pause_gameplay();
            },
            _ => {}
        }
    }

    // --------------------------------------------------------------
    //   LEVEL MANAGEMENT
    // --------------------------------------------------------------
    #[func]
    fn load_level(&mut self, level_id: i32){
        godot_print!("load_level");
        self.player = None;
        if self.current_level.is_some(){

            // clean up current map
            self.current_level.as_mut().unwrap().set_name("OldLevel");
            self.current_level.as_mut().unwrap().queue_free();

        }

        // change level
        let level_name = format!("res://scenes/levels/level_{}.tscn", level_id);
        godot_print!("Loaded {}", level_name);
        let mut current_level = load::<PackedScene>(&level_name).instantiate_as::<LevelRoot>();
        current_level.set_name("CurrentLevel");
        self.base_mut().add_child(&current_level);
        
        self.setup_level(current_level);
        // set game context
        gamestate::GameState::singleton().bind_mut().set_game_context_level(level_id);        
    }

    fn setup_level(&mut self, node: Gd<LevelRoot>){
        godot_print!("setup_level");

        self.current_level = Some(node);
        if self.current_level.is_some(){
            let mut portal: Option<Gd<Portal>> = self.current_level.as_mut().unwrap().get_node_as::<Portal>("Portal").into();
            let mut player: Option<Gd<Adventurer>> = self.current_level.as_mut().unwrap().get_node_as::<Adventurer>("Adventurer").into();

            if let Some(gd_portal) =  &mut portal{
                let on_portal_entered_callback = Callable::from_object_method(&self.base(), "on_portal_entered");
                gd_portal.connect("body_entered", &on_portal_entered_callback);
            }

            if let Some(gd_player) =  &mut player{
                let on_player_death_callback = Callable::from_object_method(&self.base(), "on_player_death");
                gd_player.connect("s_death", &on_player_death_callback);
            }
            self.player = player;

            let health_changed_callable = Callable::from_object_method(&self.hud.as_ref().unwrap(), "update_health");
            self.player.as_mut().unwrap().connect("s_health_changes", &health_changed_callable);

            /* 
            let mut interactables = self.base().try_get_node_as::<Node2D>("CurrentLevel/Interactables");
            if interactables.is_some(){
                let interaction_callback = Callable::from_object_method(&self.base(), "on_interaction_callback");
                for mut child in interactables.as_mut().unwrap().get_children().iter_shared(){
                    if child.has_signal("s_npc_interacted"){
                        child.connect("s_npc_interacted", &interaction_callback);
                    }
                }
            }
            */          
            
        }
    }

    #[func]
    fn on_main_menu_start_button_pressed(&mut self){
        godot_print!("start pressed");
        self.main_menu.as_mut().unwrap().hide();
        godot_print!("level loaded");
        gamestate::GameState::singleton().bind_mut().set_game_context_level(1);
        self.load_level(1);
        gamestate::GameState::singleton().bind_mut().resume_gameplay();
        godot_print!("resumed: {}", !gamestate::GameState::singleton().bind().is_gameplay_paused());
    }

    #[func]
    fn try_interact_deferred(&mut self) {
        self.player.as_mut().unwrap().bind_mut().try_interact();
    }        
    
}

#[godot_api]
impl INode2D for MainNode{
    fn init(base: Base<Node2D>) -> Self {
        Self{
            base: base,
            current_level: None,
            player: None,
            hud: None,
            main_menu: None,
            dialogue_box: None,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>){
        if event.is_action_pressed("pause"){
            let is_paused = gamestate::GameState::singleton().bind().is_gameplay_paused();
            if is_paused {
                gamestate::GameState::singleton().bind_mut().resume_gameplay();
            } else {
                gamestate::GameState::singleton().bind_mut().pause_gameplay();
            }
        } else if gamestate::GameState::singleton().bind().is_in_dialogue() {
            if event.is_action_pressed("interact") {
                // continue to next line
                godot_print!("In dialogue, advancing");
                DialogueSystem::singleton().bind_mut().advance();                
                //DialogueSystem::singleton().bind_mut().next_line();
                self.base_mut().get_viewport().unwrap().set_input_as_handled();
            } else if event.is_action_pressed("move_up"){
                DialogueSystem::singleton().bind_mut().move_choice_up();
            } else if event.is_action_pressed("move_down"){
                DialogueSystem::singleton().bind_mut().move_choice_down();
            }
        } else {
            if event.is_action_pressed("interact") {
                self.player.as_mut().unwrap().bind_mut().try_interact();
            }
        }
    }



    fn ready(&mut self) {
        self.hud = self.base().get_node_as::<HUD>("HUD").into(); 
        self.main_menu = self.base().get_node_as::<MainMenu>("MainMenu").into(); 
        self.dialogue_box = self.base().get_node_as::<DialogueBox>("DialogueBox").into(); 


        let on_start_button_pressed_callback = Callable::from_object_method(&self.base(), "on_main_menu_start_button_pressed");
        self.main_menu.as_mut().unwrap().connect("s_start_button_pressed", &on_start_button_pressed_callback);

        let fade_in_callable = Callable::from_object_method(&self.base(), "on_hud_fade_in_complete");
        self.hud.as_mut().unwrap().connect("s_fade_in_complete", &fade_in_callable);

        
        let fade_out_callable = Callable::from_object_method(&self.base(), "on_hud_fade_out_complete");
        self.hud.as_mut().unwrap().connect("s_fade_out_complete", &fade_out_callable);

        let dialogue_started_callable =  Callable::from_object_method(&self.dialogue_box.as_ref().unwrap(), "on_dialogue_start");
        let dialogue_ended_callable =  Callable::from_object_method(&self.dialogue_box.as_ref().unwrap(), "on_dialogue_end");
        let skip_typewriter_callable =  Callable::from_object_method(&self.dialogue_box.as_ref().unwrap(), "on_skip_typewriter");
        let choice_shown_callable =  Callable::from_object_method(&self.dialogue_box.as_ref().unwrap(), "on_choice_shown");
        let choice_highlighted_callable =  Callable::from_object_method(&self.dialogue_box.as_ref().unwrap(), "on_choice_highlighted");

        DialogueSystem::singleton().connect("s_skip_typewriter", &skip_typewriter_callable);
        DialogueSystem::singleton().connect("s_dialogue_started", &dialogue_started_callable);
        DialogueSystem::singleton().connect("s_dialogue_ended", &dialogue_ended_callable);
        DialogueSystem::singleton().connect("s_choices_shown", &choice_shown_callable);
        DialogueSystem::singleton().connect("s_choice_highlighted", &choice_highlighted_callable);

        // game state paused on initial
        gamestate::GameState::singleton().bind_mut().pause_gameplay();

    }
}
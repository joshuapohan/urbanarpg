use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};
use godot::classes::{CollisionShape2D, INode2D, Node2D};
use godot::prelude::*;
use crate::entity::adventurer::Adventurer;
use crate::template::levelroot::LevelRoot;
use crate::template::portal::Portal;


#[derive(GodotClass)]
#[class(base=Node2D)]
struct MainNode {

    base: Base<Node2D>,

    current_level: Option<Gd<LevelRoot>>,
    player: Option<Gd<Adventurer>>,
}

#[godot_api]
impl MainNode{
    #[func]
    fn on_player_death_timeout(&mut self){
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

            // change level
            let level_name = format!("res://scenes/levels/level_{}.tscn", level_id);
            godot_print!("Loaded {}", level_name);
            let mut current_level = load::<PackedScene>(&level_name).instantiate_as::<LevelRoot>();
            current_level.set_name("CurrentLevel");
            self.base_mut().add_child(&current_level);

            
            self.setup_level(current_level);
        }
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

            /* 
            let player_death_callable = Callable::from_object_method(&self.base(), "on_player_death");
            let already_connected = player.as_ref().unwrap().is_connected("s_death", &player_death_callable);
            godot_print!("already connected: {}", already_connected);
            if player.as_ref().unwrap().is_connected("s_death", &player_death_callable) {
                player.as_mut().unwrap().disconnect("s_death", &player_death_callable);
            }
            player.as_mut().unwrap().connect("s_death", &player_death_callable);
            godot_print!("connected: {}", player.as_ref().unwrap().is_connected("s_death", &player_death_callable));            
            */
        }
    }
}

#[godot_api]
impl INode2D for MainNode{
    fn init(base: Base<Node2D>) -> Self {
        Self{
            base: base,
            current_level: None,
            player: None,
        }
    }

    fn ready(&mut self) {
        let current_level_node =  self.base().get_node_as::<LevelRoot>("CurrentLevel").into();
        self.setup_level(current_level_node);
    }
}
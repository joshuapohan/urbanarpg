use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};
use godot::classes::{CollisionShape2D, INode2D, Node2D};
use godot::prelude::*;
use crate::template::levelroot::LevelRoot;
use crate::template::portal::Portal;


#[derive(GodotClass)]
#[class(base=Node2D)]
struct MainNode {

    base: Base<Node2D>,

    current_level: Option<Gd<LevelRoot>>,
}

#[godot_api]
impl MainNode{

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
        if self.current_level.is_some(){

            // clean up current map
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
            let current_level = self.current_level.as_mut().unwrap();
            let mut cs: Option<Gd<Portal>> = current_level.get_node_as::<Portal>("Portal").into();
            if cs.is_some() {
                let on_portal_entered_callback = Callable::from_object_method(&self.base(), "on_portal_entered");
                if let Some(cs) =  &mut cs{
                    cs.connect("body_entered", &on_portal_entered_callback);
                }
            }            
        }        
    }
}

#[godot_api]
impl INode2D for MainNode{
    fn init(base: Base<Node2D>) -> Self {
        Self{
            base: base,
            current_level: None,
        }
    }

    fn ready(&mut self) {
        let current_level_node =  self.base().get_node_as::<LevelRoot>("CurrentLevel").into();
        self.setup_level(current_level_node);
        /*
        self.current_level = self.base().get_node_as::<LevelRoot>("CurrentLevel").into();
        if self.current_level.is_some(){
            let current_level = self.current_level.as_mut().unwrap();
            let mut cs: Option<Gd<Portal>> = current_level.get_node_as::<Portal>("Portal").into();
            if cs.is_some() {
                let on_portal_entered_callback = Callable::from_object_method(&self.base(), "on_portal_entered");
                if let Some(cs) =  &mut cs{
                    cs.connect("body_entered", &on_portal_entered_callback);
                }
            }            
        }
        */
    }
}
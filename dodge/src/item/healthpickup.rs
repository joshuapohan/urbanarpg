use godot::register::GodotClass;
use godot::prelude::*;
use godot::classes::{Area2D, IArea2D};

use crate::entity::adventurer::Adventurer;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct HealthPickup{
    #[export]
    heal_amount: i32,

    base: Base<Area2D>,
}

#[godot_api]
impl HealthPickup{
    #[func]
    fn on_body_entered(&mut self,  body: Gd<Node2D>){
        if  body.get_name().contains("Adventurer"){
            if let Ok(mut adventurer) = body.try_cast::<Adventurer>(){
                godot_print!("Adventurer Health Pickup");
                {
                    let mut bind_adventurer = adventurer.bind_mut();
                    bind_adventurer.heal(self.heal_amount);
                    self.base_mut().queue_free();
                }
            }            
        }
    }
}

#[godot_api]
impl IArea2D for HealthPickup{
    fn init(base: Base<Area2D>) -> Self {

        Self{
            heal_amount: 10,
            base: base,
        }
    }

    fn ready(&mut self){

        // Initialize hitbox callbacks
        let on_body_entered_callback = Callable::from_object_method(&self.base(), "on_body_entered");
        self.base_mut().connect("body_entered", &on_body_entered_callback);
    }

    fn physics_process(&mut self, _delta: f64){
    }
}
use godot::register::GodotClass;
use godot::prelude::*;
use godot::classes::{Area2D, AudioStreamPlayer2D, IArea2D};

use crate::entity::adventurer::Adventurer;
use crate::item::basepickup::BasePickup;
use crate::log_info;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct HealthPickup{
    #[export]
    heal_amount: i32,

    base: Base<Area2D>,
    base_pickup: BasePickup, 
}

#[godot_api]
impl HealthPickup{

    #[func]
    fn on_pickup_audio_finished(&mut self){
        log_info!("Pickup audio finished");
        self.base_mut().queue_free();
    }

    #[func]
    fn on_body_entered(&mut self,  body: Gd<Node2D>){
        if  body.get_name().contains("Adventurer"){
            if let Ok(mut adventurer) = body.try_cast::<Adventurer>(){
                log_info!("Adventurer Health Pickup");
                let mut bind_adventurer = adventurer.bind_mut();
                bind_adventurer.heal(self.heal_amount);
                self.base_pickup.on_pickup(self.to_gd().upcast::<Area2D>());                
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
            base_pickup: BasePickup{
                pickup_audio: None,
            }
        }
    }

    fn ready(&mut self){
        self.base_pickup.pickup_audio = self.base().get_node_as::<AudioStreamPlayer2D>("PickupAudio").into();

        // Initialize hitbox callbacks
        let on_body_entered_callback = Callable::from_object_method(&self.base(), "on_body_entered");
        self.base_mut().connect("body_entered", &on_body_entered_callback);

        let on_pickup_audio_finished_callback = Callable::from_object_method(&self.base(), "on_pickup_audio_finished");
        self.base_pickup.pickup_audio.as_mut().unwrap().connect("finished", &on_pickup_audio_finished_callback);
    }

    fn physics_process(&mut self, _delta: f64){
    }
}
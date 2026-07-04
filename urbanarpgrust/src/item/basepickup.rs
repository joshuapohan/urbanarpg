use godot::prelude::*;
use godot::classes::AudioStreamPlayer2D;
use godot::classes::Area2D;

pub struct BasePickup {    
    pub pickup_audio: Option<Gd<AudioStreamPlayer2D>>,    
}

impl BasePickup{
    pub fn on_pickup(&mut self, mut base: Gd<Area2D>){
        base.hide();
        base.set_deferred("monitoring", &false.to_variant());
        base.set_deferred("monitorable", &false.to_variant());
        self.pickup_audio.as_mut().unwrap().play();
    }
}
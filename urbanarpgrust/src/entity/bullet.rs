use godot::register::GodotClass;
use godot::prelude::*;
use godot::classes::{Area2D, AudioStreamPlayer2D, IArea2D, Texture2D};

use crate::entity::slime::Slime;
use crate::log_info;
use crate::script::gamestate;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Bullet {
    base: Base<Area2D>,
    direction: Vector2,
    pub is_active: bool,

    #[export]
    damage: i32,
    speed: f32,
}

#[godot_api]
impl Bullet {
    pub fn activate(&mut self, position: Vector2, direction: Vector2, damage: i32) {
        self.is_active = true;
        self.direction = direction;
        self.damage = damage;
        self.base_mut().set_position(position);
        self.base_mut().show();
        self.base_mut().set_deferred("monitoring", &true.to_variant());
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.base_mut().hide();
        self.base_mut().set_deferred("monitoring", &false.to_variant());
    }

    #[func]
    fn on_body_entered(&mut self,  body: Gd<Node2D>){
        if body.get_name().contains("Slime"){
            if let Ok(mut slime) = body.try_cast::<Slime>(){
                log_info!("Slime Hit");
                let mut bind_slime = slime.bind_mut();
                bind_slime.take_damage(self.damage, self.base().get_position());
            }
            self.deactivate();
            return;
        } else if body.get_name().contains("Adventurer"){
            return
        }
        self.deactivate();
    }

}


#[godot_api]
impl IArea2D for Bullet {
    fn init(base: Base<Area2D>) -> Self {    
        return Self {
            base,
            direction: Vector2::ZERO,
            damage: 30,
            speed: 1000.0,
            is_active: false
        }
    }


    fn ready(&mut self){
        // Initialize nodes
        let on_body_entered_callback = Callable::from_object_method(&self.base(), "on_body_entered");
        self.base_mut().connect("body_entered", &on_body_entered_callback);
    }

    fn physics_process(&mut self, delta: f64){
        if gamestate::GameState::singleton().bind().is_gameplay_paused() {
            return
        }

        if gamestate::GameState::singleton().bind().is_in_dialogue() {
            return
        }
        
        let velocity = self.direction * self.speed * delta as f32;
        let pos = self.base().get_position() + velocity;
        self.base_mut().set_position(pos);        
    }    
}
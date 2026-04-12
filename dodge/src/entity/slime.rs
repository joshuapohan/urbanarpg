use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Slime{
    #[export]
    speed: f32,
    velocity: Vector2,

    animated_sprite: Option<Gd<AnimatedSprite2D>>,
    sight: Option<Gd<Area2D>>,
    target: Option<Gd<Node2D>>,

    base: Base<CharacterBody2D>
}

#[godot_api]
impl Slime {
    fn play_animation(&mut self, velocity: Vector2){
        if velocity.length() > 0.0 {
        } else {
            self.animated_sprite.as_mut().unwrap().set_animation("idle");
            self.animated_sprite.as_mut().unwrap().play();
        }        
    }

    #[func]
    fn on_sight_entered(&mut self, body: Gd<Node2D>){
        godot_print!("player entered");

        if body.get_name() == "Adventurer"{
            godot_print!("Adventurer entered");
            self.target = Some(body);
        }
    }

    #[func]
    fn on_sight_exited(&mut self, body: Gd<Node2D>){
        godot_print!("player exited");

        if body.get_name() == "Adventurer"{
            godot_print!("Adventurer exited");
            self.target = None;
        }
    }
    
    #[func]
    fn attack(&mut self, delta: f64){
        if self.target.as_ref().unwrap().is_instance_valid(){
            let direction = self.speed *  (self.target.as_ref().unwrap().get_position() - self.base().get_position()).normalized();
            let current_pos = self.base().get_position();
            let new_pos = (current_pos + direction * delta as f32);            
            self.base_mut().set_position(new_pos);

            self.animated_sprite.as_mut().unwrap().set_animation("attack");
            self.animated_sprite.as_mut().unwrap().play();
        }
    }

}

#[godot_api]
impl ICharacterBody2D for Slime{
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self{
            base: base,

            animated_sprite: None,
            sight: None,
            target: None,

            velocity: Vector2::ZERO,
            speed: 100.0,
        }
    }

    

    fn ready(&mut self){

        // Initialize nodes
        self.animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").into();
        self.sight = self.base().get_node_as::<Area2D>("Sight").into();


        // Initialize signals
        let on_sight_entered_callback = Callable::from_object_method(&self.base(), "on_sight_entered");
        if let Some(sight) = &mut self.sight{
            sight.connect("body_entered", &on_sight_entered_callback);
        }

        let on_sight_exited_callback = Callable::from_object_method(&self.base(), "on_sight_exited");
        if let Some(sight) = &mut self.sight{
            sight.connect("body_exited", &on_sight_exited_callback);
        }        
    }

    fn physics_process(&mut self, delta: f64){
        if self.target.is_some() && self.target.as_ref().unwrap().is_instance_valid(){
            self.attack(delta);
        } else {
            self.play_animation(self.velocity);
        }
    }
}
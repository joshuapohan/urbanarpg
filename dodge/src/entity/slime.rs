use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Slime{
    velocity: Vector2,

    animated_sprite: Option<Gd<AnimatedSprite2D>>,

    base: Base<CharacterBody2D>
}

impl Slime {



    fn play_animation(&mut self, velocity: Vector2){
        if velocity.length() > 0.0 {
        } else {
            self.animated_sprite.as_mut().unwrap().set_animation("idle");
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
            velocity: Vector2::ZERO,
        }
    }

    

    fn ready(&mut self){

        // Initialize nodes
        self.animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").into();

    }

    fn process(&mut self, delta: f64){
        let v= self.velocity;
        self.play_animation(self.velocity);
    }
}
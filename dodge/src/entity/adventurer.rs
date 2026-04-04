use godot::classes::AnimatedSprite2D;
use godot::classes::Input;
use godot::obj::Base;
use godot::prelude::*;
use godot::classes::{CharacterBody2D, ICharacterBody2D};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Adventurer {
    #[export]
    speed: f32,

    animated_sprite: Option<Gd<AnimatedSprite2D>>,

    base: Base<CharacterBody2D>    
}

#[godot_api]
impl Adventurer{

    fn play_animation(&mut self, velocity: Vector2){
        if velocity.length() > 0.0 {
            if velocity.y < 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation("run_up");
                self.animated_sprite.as_mut().unwrap().set_flip_h(false);
            } else if velocity.y > 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation("run_down");
                self.animated_sprite.as_mut().unwrap().set_flip_h(false);
            } else if velocity.x > 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation("run_right");
                self.animated_sprite.as_mut().unwrap().set_flip_h(false);
            } else if velocity.x < 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation("run_right");
                self.animated_sprite.as_mut().unwrap().set_flip_h(true);
            }
            
            self.animated_sprite.as_mut().unwrap().play();
        } else {
            self.animated_sprite.as_mut().unwrap().stop();
        }        
    }    
}

#[godot_api]
impl ICharacterBody2D for Adventurer{
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self{
            speed:50.0,
            base: base,
            animated_sprite: None,
        }
    }

    fn ready(&mut self){
        self.animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").into();
    }

    fn process(&mut self, delta: f64){
        let input = Input::singleton();
        let mut velocity = input.get_vector("move_left", "move_right", "move_up", "move_down");
        velocity = velocity.normalized() * self.speed;

        self.play_animation(velocity);
        let current_pos = self.base().get_position();
        let new_pos = (current_pos + velocity * delta as f32);
        self.base_mut().set_position(new_pos);
    }    
}
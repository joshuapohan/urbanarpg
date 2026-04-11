use godot::classes::AnimatedSprite2D;
use godot::classes::Area2D;
use godot::classes::AudioStreamPlayer2D;
use godot::classes::Input;
use godot::obj::Base;
use godot::prelude::*;
use godot::classes::{CharacterBody2D, ICharacterBody2D};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Adventurer {
    #[export]
    speed: f32,

    last_direction: Vector2,
    is_attacking: bool,
    hitbox_offset: Vector2,

    animated_sprite: Option<Gd<AnimatedSprite2D>>,
    swing_sword_audio: Option<Gd<AudioStreamPlayer2D>>,
    hitbox_area: Option<Gd<Area2D>>,

    base: Base<CharacterBody2D>    
}

#[godot_api]
impl Adventurer{

    fn process_movement(&mut self, delta: f64){
        let input = Input::singleton();

        if input.is_action_just_pressed("attack(physical)") && !self.is_attacking{
            self.attack();
        }

        // skip movement if is attacking
        if self.is_attacking{
            return;
        }        

        let mut velocity = input.get_vector("move_left", "move_right", "move_up", "move_down");

        if velocity.length() > 0.0 {
            self.last_direction = velocity.normalized();
            velocity = velocity.normalized() * self.speed;
        }


        self.update_hitbox_offset();

        self.process_animation(velocity);
        let current_pos = self.base().get_position();
        let new_pos = (current_pos + velocity * delta as f32);

        self.base_mut().set_position(new_pos);
    }

    fn process_animation(&mut self, velocity: Vector2){
        if self.is_attacking {
             return;
        }
        let (prefix, v) = if velocity == Vector2::ZERO {
            ("idle".to_string(), self.last_direction)
        } else {
            ("run".to_string(), velocity)
        };
        self.play_animation(prefix, v);
    }

    fn play_animation(&mut self, prefix: String, velocity: Vector2){
        if velocity.length() > 0.0 {
            if velocity.y < 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation(( prefix+ "_up").as_str());
                self.animated_sprite.as_mut().unwrap().set_flip_h(false);
            } else if velocity.y > 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation((prefix+"_down").as_str());
                self.animated_sprite.as_mut().unwrap().set_flip_h(false);
            } else if velocity.x > 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation((prefix+"_right").as_str());
                self.animated_sprite.as_mut().unwrap().set_flip_h(false);
            } else if velocity.x < 0.0 {
                self.animated_sprite.as_mut().unwrap().set_animation((prefix+"_right").as_str());
                self.animated_sprite.as_mut().unwrap().set_flip_h(true);
            }
            
            self.animated_sprite.as_mut().unwrap().play();
        } else {
            self.animated_sprite.as_mut().unwrap().stop();
        }        
    }

    // --------------------------------------------------------------
    //    ATTACKING
    // --------------------------------------------------------------
    fn attack(&mut self) {
        self.is_attacking = true;
        self.swing_sword_audio.as_mut().unwrap().play();
        let last_v = self.last_direction;
        self.play_animation("attack".to_string(), last_v);
    }

    // --------------------------------------------------------------
    //    HITBOX
    // --------------------------------------------------------------
    fn update_hitbox_offset(&mut self){
        let x = self.hitbox_offset.x;
        let y = self.hitbox_offset.y;

        godot_print!("{:?}", self.last_direction);
        if self.last_direction == Vector2::LEFT {
            self.hitbox_area.as_mut().unwrap().set_position(Vector2 { x: -x, y: y });
        } else if self.last_direction == Vector2::RIGHT {
            self.hitbox_area.as_mut().unwrap().set_position(Vector2 { x: x, y: y });
        } else if self.last_direction == Vector2::UP {
            self.hitbox_area.as_mut().unwrap().set_position(Vector2 { x: y, y: -x });            
        } else if self.last_direction == Vector2::DOWN {
            self.hitbox_area.as_mut().unwrap().set_position(Vector2 { x: y, y: x });        
        }
    }


    #[func]    
    fn on_animation_finish(&mut self){
        self.is_attacking = false;
    }
}

#[godot_api]
impl ICharacterBody2D for Adventurer{
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self{
            speed:50.0,
            hitbox_offset: Vector2::ZERO,
            base: base,
            animated_sprite: None,
            swing_sword_audio: None,
            hitbox_area: None,
            last_direction: Vector2::RIGHT,
            is_attacking: false,
        }
    }

    fn ready(&mut self){

        // Initialize nodes
        self.animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").into();
        self.swing_sword_audio = self.base().get_node_as::<AudioStreamPlayer2D>("SwingSword").into();
        self.hitbox_area = self.base().get_node_as::<Area2D>("Hitbox").into();

        // Initialize signal callbacks
        let animation_finish_callable = Callable::from_object_method(&self.base(), "on_animation_finish");
        if let Some(animated_sprite) = &mut self.animated_sprite{
            animated_sprite.connect("animation_finished", &animation_finish_callable);
        }

        // Initialize hitbox
        self.hitbox_offset = self.hitbox_area.as_ref().unwrap().get_position();
    }

    fn process(&mut self, delta: f64){
        self.process_movement(delta);

        self.base_mut().move_and_slide();
    }    
}
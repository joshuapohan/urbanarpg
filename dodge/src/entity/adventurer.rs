use godot::classes::AnimatedSprite2D;
use godot::classes::Area2D;
use godot::classes::AudioStreamPlayer2D;
use godot::classes::Input;
use godot::classes::Timer;
use godot::obj::Base;
use godot::prelude::*;
use godot::classes::{CharacterBody2D, ICharacterBody2D};

use crate::entity::slime::Slime;
use crate::script::playerstats::PlayerStats;


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Adventurer {
    #[export]
    speed: f32,
    #[export]
    strength: i32,
    #[export]
    health: i32,
    #[export]
    max_health: i32,


    last_direction: Vector2,
    is_attacking: bool,
    is_invincible: bool,
    hitbox_offset: Vector2,

    animated_sprite: Option<Gd<AnimatedSprite2D>>,
    swing_sword_audio: Option<Gd<AudioStreamPlayer2D>>,
    take_damage_audio: Option<Gd<AudioStreamPlayer2D>>,
    hitbox_area: Option<Gd<Area2D>>,
    damage_cooldown_timer: Option<Gd<Timer>>,


    base: Base<CharacterBody2D>    
}

#[godot_api]
impl Adventurer{

    fn process_movement(&mut self){
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
        self.base_mut().set_velocity(velocity);
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
        self.hitbox_area.as_mut().unwrap().set_monitoring(true);
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

    #[func]
    fn on_body_entered(&mut self,  body: Gd<Node2D>){
        if self.is_attacking && body.get_name().contains("Slime"){
            if let Ok(mut slime) = body.try_cast::<Slime>(){
                godot_print!("Slime Hit");
                let mut bind_slime = slime.bind_mut();
                bind_slime.take_damage(self.strength, self.base().get_position());
            }            
        }
    }

    pub fn take_damage(&mut self, damage: i32, attacker_position: Vector2){
        if self.is_invincible {
            return
        }
        self.damage_cooldown_timer.as_mut().unwrap().start();
        self.take_damage_audio.as_mut().unwrap().play();
        self.health -= damage;
        PlayerStats::singleton().bind_mut().health -= damage;
        godot_print!("{}", self.health);
        if self.health <= 0 {
        
        } 
        else {
            self.is_invincible = true;
        }
    }
    
    #[func]
    fn on_damage_cooldown_timer_timeout(&mut self){
        self.is_invincible = false;
        self.damage_cooldown_timer.as_mut().unwrap().stop();
    }        
}

#[godot_api]
impl ICharacterBody2D for Adventurer{
    fn init(base: Base<CharacterBody2D>) -> Self {
        let singleton= PlayerStats::singleton();
        let binding = singleton.bind();

        Self{
            speed:50.0,
            strength: 10,
            hitbox_offset: Vector2::ZERO,
            base: base,
            animated_sprite: None,
            swing_sword_audio: None,
            take_damage_audio: None,
            hitbox_area: None,
            damage_cooldown_timer: None,
            last_direction: Vector2::RIGHT,
            is_attacking: false,
            is_invincible: false,
            max_health: binding.max_health,
            health: binding.health,
        }
    }

    fn ready(&mut self){

        // Initialize nodes
        self.animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").into();
        self.swing_sword_audio = self.base().get_node_as::<AudioStreamPlayer2D>("SwingSwordAudio").into();
        self.take_damage_audio = self.base().get_node_as::<AudioStreamPlayer2D>("TakeDamageAudio").into();
        self.damage_cooldown_timer =  self.base().get_node_as::<Timer>("DamageCooldownTimer").into();
        self.hitbox_area = self.base().get_node_as::<Area2D>("Hitbox").into();

        // Initialize signal callbacks
        let animation_finish_callable = Callable::from_object_method(&self.base(), "on_animation_finish");
        if let Some(animated_sprite) = &mut self.animated_sprite{
            animated_sprite.connect("animation_finished", &animation_finish_callable);
        }

        // Initialize hitbox callbacks
        let on_body_entered_callback = Callable::from_object_method(&self.base(), "on_body_entered");
        if let Some(hitbox) = &mut self.hitbox_area{
            hitbox.connect("body_entered", &on_body_entered_callback);
        }

        // Initialize hitbox
        self.hitbox_offset = self.hitbox_area.as_ref().unwrap().get_position();

        // timer callbacks
        let on_damage_cooldown_timer_callback = Callable::from_object_method(&self.base(), "on_damage_cooldown_timer_timeout");
        if let Some(damage_cooldown_timer) = &mut self.damage_cooldown_timer{
            damage_cooldown_timer.connect("timeout", &on_damage_cooldown_timer_callback);
        }        
    }

    fn physics_process(&mut self, _delta: f64){
        self.hitbox_area.as_mut().unwrap().set_monitoring(false);

        self.process_movement();

        self.base_mut().move_and_slide();
    }
}
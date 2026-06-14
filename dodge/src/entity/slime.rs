use godot::classes::tween::{EaseType, TransitionType};
use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Area2D, AudioStreamPlayer2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, Timer, Tween};

use crate::entity::adventurer::Adventurer;
use crate::ui::healthbar::HealthBar;


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Slime{
    #[export]
    speed: f32,
    #[export]
    health: i32,
    #[export]
    strength: i32,
    #[export]
    knockback_force: f32,
    #[export]
    alive: bool,
    
    animated_sprite: Option<Gd<AnimatedSprite2D>>,
    take_damage_audio: Option<Gd<AudioStreamPlayer2D>>,
    health_bar_ui: Option<Gd<HealthBar>>,
    attack_timer: Option<Gd<Timer>>,
    
    sight: Option<Gd<Area2D>>,
    target: Option<Gd<Node2D>>,
    target_in_attack_range: Option<Gd<Adventurer>>,
    hitbox_area: Option<Gd<Area2D>>,
    
    
    
    base: Base<CharacterBody2D>
}

#[godot_api]
impl Slime {
    fn play_animation(&mut self){
        if self.alive {
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
            let new_pos = current_pos + direction * delta as f32;            
            self.base_mut().set_position(new_pos);
            
            self.animated_sprite.as_mut().unwrap().set_animation("attack");
            self.animated_sprite.as_mut().unwrap().play();
        }
    }
    
    pub fn take_damage(&mut self, damage: i32, attacker_position: Vector2){
        self.health -= damage;
        
        let hb = self.health_bar_ui.as_mut().unwrap();
        hb.bind_mut().update_health(self.health);

        if self.health <= 0 {
            self.die();
        } else {            
            self.take_damage_audio.as_mut().unwrap().play();
            
            let knockback_direction =   (self.base().get_position() - attacker_position).normalized();
            let current_pos = self.base().get_position();
            let new_pos = current_pos + knockback_direction * self.knockback_force;     
            
            let obj: Gd<Object> = self.to_gd().upcast::<Object>();
            let mut tween = self.base_mut().create_tween();
            tween.set_ease(EaseType::IN);
            tween.set_trans(TransitionType::CUBIC);
            tween.tween_property(
                &obj,
                "position",
                &new_pos.to_variant(),
                0.15,
            );
        }        
    }

    pub fn drop_item(){

    }
    
    fn die(&mut self){
        self.alive = false;
        self.animated_sprite.as_mut().unwrap().set_animation("die");
        self.animated_sprite.as_mut().unwrap().play();
        self.take_damage_audio.as_mut().unwrap().set_pitch_scale(0.5);
        self.take_damage_audio.as_mut().unwrap().play();

        // disable collision
        if let Some(node) = self.base().get_node_or_null("CollisionShape2D"){
            let mut collision_2d: Gd<CollisionShape2D> = node.try_cast().unwrap();
            collision_2d.set_deferred("disabled", &Variant::from(true));
        } else {
            godot_error!("Unable to find collision node for slime")
        }
        if let Some(node) = self.base().get_node_or_null("Sight/CollisionShape2D"){
            let mut collision_2d: Gd<CollisionShape2D> = node.try_cast().unwrap();
            collision_2d.set_deferred("disabled", &Variant::from(true));
        } else {
            godot_error!("Unable to find collision node for slime")
        }
        if let Some(node) = self.base().get_node_or_null("Hitbox/CollisionShape2D"){
            let mut collision_2d: Gd<CollisionShape2D> = node.try_cast().unwrap();
            collision_2d.set_deferred("disabled", &Variant::from(true));
        } else {
            godot_error!("Unable to find collision node for slime")
        }             
    }
    
    #[func]
    fn on_body_entered(&mut self,  body: Gd<Node2D>){
        if  body.get_name().contains("Adventurer"){
            if let Ok(mut adventurer) = body.try_cast::<Adventurer>(){
                godot_print!("Adventurer Hit");
                {
                    let mut bind_adventurer = adventurer.bind_mut();
                    bind_adventurer.take_damage(self.strength, self.base().get_position());
                }
                self.target_in_attack_range = Some(adventurer.clone());
                self.attack_timer.as_mut().unwrap().start();
            }            
        }
    }

    #[func]
    fn on_body_exited(&mut self,  body: Gd<Node2D>){
        if body.instance_id() == self.target_in_attack_range.as_ref().unwrap().instance_id() {
            self.target_in_attack_range = None;
            self.attack_timer.as_mut().unwrap().stop();
        }
    }    
    
    #[func]
    fn on_attack_timer_timeout(&mut self){
        if self.target_in_attack_range.is_some(){
            let self_pos = self.base().get_position();
            let damage = self.strength;
            let mut bind_adventurer = self.target_in_attack_range.as_mut().unwrap().bind_mut();
            bind_adventurer.take_damage(damage, self_pos);
        }
    }    
}

#[godot_api]
impl ICharacterBody2D for Slime{
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self{
            base: base,
            
            animated_sprite: None,
            take_damage_audio: None,
            sight: None,
            target: None,
            health_bar_ui: None,
            hitbox_area: None,
            target_in_attack_range: None,
            attack_timer: None,
            
            speed: 100.0,
            health: 100,
            knockback_force: 30.0,
            strength: 10,
            alive: true,
        }
    }
    
    
    
    fn ready(&mut self){
        
        // Initialize nodes
        self.animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D").into();
        self.sight = self.base().get_node_as::<Area2D>("Sight").into();
        self.take_damage_audio = self.base().get_node_as::<AudioStreamPlayer2D>("TakeDamage").into();
        self.health_bar_ui = self.base().get_node_as::<HealthBar>("HealthBar").into();
        self.hitbox_area = self.base().get_node_as::<Area2D>("Hitbox").into();
        self.attack_timer = self.base().get_node_as::<Timer>("AttackTimer").into();
        
        
        // Initialize signals
        let on_sight_entered_callback = Callable::from_object_method(&self.base(), "on_sight_entered");
        if let Some(sight) = &mut self.sight{
            sight.connect("body_entered", &on_sight_entered_callback);
        }
        
        let on_sight_exited_callback = Callable::from_object_method(&self.base(), "on_sight_exited");
        if let Some(sight) = &mut self.sight{
            sight.connect("body_exited", &on_sight_exited_callback);
        }
        
        // Initialize hitbox callbacks
        let on_body_entered_callback = Callable::from_object_method(&self.base(), "on_body_entered");
        if let Some(hitbox) = &mut self.hitbox_area{
            hitbox.connect("body_entered", &on_body_entered_callback);
        }

        let on_body_exited_callback = Callable::from_object_method(&self.base(), "on_body_exited");
        if let Some(hitbox) = &mut self.hitbox_area{
            hitbox.connect("body_exited", &on_body_exited_callback);
        }

        // timer callbacks
        let on_attack_timer_callback = Callable::from_object_method(&self.base(), "on_attack_timer_timeout");
        if let Some(attack_timer) = &mut self.attack_timer{
            attack_timer.connect("timeout", &on_attack_timer_callback);
        }
    }
    
    fn physics_process(&mut self, delta: f64){
        if self.alive && self.target.is_some() && self.target.as_ref().unwrap().is_instance_valid(){
            self.attack(delta);
        } else {
            self.play_animation();
        }
    }
}
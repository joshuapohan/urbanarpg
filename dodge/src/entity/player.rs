use godot::classes::AnimatedSprite2D;
use godot::classes::CollisionShape2D;
use godot::classes::DisplayServer;
use godot::classes::Input;
use godot::prelude::*;
use godot::classes::Area2D;
use godot::classes::IArea2D;
use godot::classes::Node2D;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    screen_size: Vector2,

    #[export]
    speed: f32,

    #[export]
    dummy: f32,

    #[base]
    base: Base<Area2D>
}

#[godot_api]
impl Player{
    #[signal]
    fn hit();

    fn on_hit(&mut self, body: Gd<Node2D>){
        godot_print!("Player Hit");
        self.base_mut().hide();
        self.signals().hit().emit();
        if let Some(node) = self.base().get_node_or_null("CollisionShape2D"){
            let mut collision_2d: Gd<CollisionShape2D> = node.try_cast().unwrap();
            collision_2d.set_deferred("disabled", &Variant::from(true));
        } else {
            godot_error!("Unable to find collision node for player")
        }
    }

    #[func]
    fn start(&mut self, pos: Vector2){
        self.base_mut().set_position(pos);
        self.base_mut().show();
        if let Some(node) = self.base().get_node_or_null("CollisionShape2D"){
            let mut collision_2d: Gd<CollisionShape2D> = node.try_cast().unwrap();
            collision_2d.set_deferred("disabled", &Variant::from(false));
        } else {
            godot_error!("Unable to find collision node for player")
        }        
    }
}

#[godot_api]
impl IArea2D for Player {

    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Player Initialized");

        Self {speed:10.0,base, screen_size: Vector2::ZERO, dummy: 0.0}
    }

    fn ready(&mut self) {
        let ds = DisplayServer::singleton();
        self.screen_size = ds.screen_get_size().cast_float();
        godot_print!("Screen size: {:?}", self.screen_size);
        self.signals().body_entered().connect_self(Self::on_hit);
        //self.signals().hit().connect_self(Self::on_hit);
        self.base_mut().hide();
    }

    fn process(&mut self, delta: f64){

        let mut velocity = Vector2::ZERO;
        let input = Input::singleton();

        if Input::is_action_pressed(&input, "move_right"){
            velocity.x += 1.0;
        }
        if Input::is_action_pressed(&input, "move_left"){
            velocity.x += -1.0;
        }
        if Input::is_action_pressed(&input, "move_up"){
            velocity.y += -1.0;
        }
        if Input::is_action_pressed(&input, "move_down"){
            velocity.y += 1.0;
        }

        if let Some(sprite_child) = self.base().get_node_or_null("AnimatedSprite2D"){
            let mut animated_sprite: Gd<AnimatedSprite2D> = sprite_child.try_cast().unwrap();
            if velocity.length() > 0.0 {
                velocity = velocity.normalized() * self.speed;
                if velocity.y != 0.0 {
                    animated_sprite.set_animation("up");
                    animated_sprite.set_flip_v(velocity.y > 0.0);
                } else if velocity.x != 0.0 {
                    animated_sprite.set_animation("walk");
                    animated_sprite.set_flip_h(velocity.x < 0.0);
                    animated_sprite.set_flip_v(false);
                }
                animated_sprite.play();
            } else {
                animated_sprite.stop();
            }
            let current_pos = self.base().get_position();
            let new_pos = (current_pos + velocity * delta as f32).clamp(Vector2::ZERO, self.screen_size);
            self.base_mut().set_position(new_pos);
        } else {
            godot_error!("Error, child node AnimatedSprite2D not found!");
            return
        }                 
    }
}
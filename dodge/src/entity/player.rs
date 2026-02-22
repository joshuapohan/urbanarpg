use godot::classes::AnimatedSprite2D;
use godot::classes::DisplayServer;
use godot::classes::Input;
use godot::prelude::*;
use godot::classes::Area2D;
use godot::classes::IArea2D;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    screen_size: Vector2,

    #[export]
    speed: f32,

    #[base]
    base: Base<Area2D>
}

#[godot_api]
impl IArea2D for Player {

    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello World");

        Self {speed:10.0,base, screen_size: Vector2::ZERO}
    }

    fn ready(&mut self) {
        let ds = DisplayServer::singleton();
        self.screen_size = ds.screen_get_size().cast_float();
        godot_print!("Screen size: {:?}", self.screen_size);
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
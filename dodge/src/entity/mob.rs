use godot::classes::IRigidBody2D;
use godot::classes::RigidBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
struct Mob {
    screen_size: Vector2,

    #[base]
    base: Base<RigidBody2D>
}

#[godot_api]
impl IRigidBody2D for Mob {

    fn init(base: Base<RigidBody2D>) -> Self {
        godot_print!("Mob Initialized");

        Self {base, screen_size: Vector2::ZERO}
    }

}
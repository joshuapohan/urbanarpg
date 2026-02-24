use godot::classes::IRigidBody2D;
use godot::classes::RigidBody2D;
use godot::classes::AnimatedSprite2D;
use godot::classes::VisibleOnScreenNotifier2D;
use godot::prelude::*;
use rand;
use rand::Rng;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
struct Mob {
    #[base]
    base: Base<RigidBody2D>
}

impl Mob{

    fn on_screen_exit(&mut self){
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IRigidBody2D for Mob {

    fn init(base: Base<RigidBody2D>) -> Self {
        godot_print!("Mob Initialized");

        Self {base}
    }

    fn ready(&mut self){
        if let Some(sprite_child) = self.base().get_node_or_null("AnimatedSprite2D"){
            // Initialize animation frames
            let mut rng = rand::thread_rng();

            let mut animated_sprite: Gd<AnimatedSprite2D> = sprite_child.try_cast().unwrap();
            let mob_types = animated_sprite.get_sprite_frames().unwrap().get_animation_names().to_vec();

            let random_string_index: usize = rng.gen_range(0..mob_types.len());
            let animation = mob_types.get(random_string_index).unwrap();

            animated_sprite.set_animation(&StringName::from(animation));
            animated_sprite.play();

            // Initialize signal
            if let Some(gd_node) = self.base().get_node_or_null("VisibleOnScreenNotifier2D"){
                let mut visible_on_screen_notifier_2d: Gd<VisibleOnScreenNotifier2D> = gd_node.try_cast().unwrap();
                visible_on_screen_notifier_2d.connect("screen_exited", &Callable::from_object_method(&self.base(), "on_screen_exit"));
            }

        } else {
            godot_error!("Animated Sprite 2D not found")
        }
    }

}
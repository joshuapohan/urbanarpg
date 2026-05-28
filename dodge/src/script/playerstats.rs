use godot::prelude::*;

#[derive(GodotClass)]
#[class(singleton)]
pub struct PlayerStats{
    pub health: i32,
    pub max_health: i32,
    base: Base<Object>,
}

#[godot_api]
impl PlayerStats {
    #[func]
    pub fn reset(&mut self){
        self.health = self.max_health;
    }
}

#[godot_api]
impl IObject for PlayerStats{
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            health: 10,
            max_health: 10,
        }
    }
}
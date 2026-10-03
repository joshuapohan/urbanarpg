use godot::register::GodotClass;
use godot::prelude::*;
use godot::classes::{Area2D, AudioStreamPlayer2D, IArea2D, Texture2D};

use crate::entity::bullet::Bullet;
use crate::script::gamestate;

#[derive(GodotClass)]
#[class(singleton)]
pub struct BulletPool {
    base: Base<Object>,
    pool: Vec<Gd<Bullet>>,

    #[export]
    pool_size: i32

}

#[godot_api]
impl BulletPool {
    pub fn initialize(&mut self, parent: Gd<Node>, pool_size: i32) {
        let bullet_scene = load::<PackedScene>("res://scenes/bullet.tscn");
        self.pool.clear();
        for _ in 0..pool_size {
            let mut bullet = bullet_scene.instantiate_as::<Bullet>();
            bullet.bind_mut().deactivate();
            parent.clone().add_child(&bullet);
            self.pool.push(bullet);
        }
    }
    
    pub fn get_bullet(&mut self) -> Option<Gd<Bullet>>{
        self.pool.iter()
            .find(|b| !b.bind().is_active)
            .cloned()
    }

    pub fn fire(&mut self, position: Vector2, direction: Vector2, damage: i32){
        if let Some(mut bullet) = self.get_bullet(){
            bullet.bind_mut().activate(position, direction, damage);
        }
    }
}


#[godot_api]
impl IObject for BulletPool {
    fn init(base: Base<Object>) -> Self { 
        return Self {
            base,        
            pool_size: 30,
            pool: Vec::<Gd<Bullet>>::new(),
        };
    }
}
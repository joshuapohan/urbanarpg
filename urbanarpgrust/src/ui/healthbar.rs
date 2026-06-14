use godot::prelude::*;
use godot::classes::{INode2D, Node2D, Sprite2D};


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct HealthBar{
    health_under_sprite_2d: Option<Gd<Sprite2D>>,
    health_sprite_2d: Option<Gd<Sprite2D>>,
    default_width: f32,
    default_height: f32,
    
    base: Base<Node2D>
}

#[godot_api]
impl HealthBar {
    pub fn update_health(&mut self, new_health: i32){
        if let Some(sprite) = &mut self.health_sprite_2d {
            let new_width = (new_health as f32 / 100.0) * self.default_width;
            sprite.set_region_rect(Rect2 {position:Vector2::ZERO,size: Vector2{ x: new_width, y: self.default_height } });
        }        
    }        
}

#[godot_api]
impl INode2D for HealthBar{
    fn init(base: Base<Node2D>) -> Self {
        Self{
            base: base,
            
            health_under_sprite_2d: None,
            health_sprite_2d: None,

            default_width: 0.0,
            default_height: 0.0,
        }
    }
    
    
    
    fn ready(&mut self){
        
        // Initialize nodes
        self.health_under_sprite_2d = self.base().get_node_as::<Sprite2D>("HealthUnder").into();     
        self.health_sprite_2d = self.base().get_node_as::<Sprite2D>("Health").into();  
        self.default_height = self.health_sprite_2d.as_ref().unwrap().get_texture().unwrap().get_size().y; 
        self.default_width = self.health_sprite_2d.as_ref().unwrap().get_texture().unwrap().get_size().x; 
    }
    
}
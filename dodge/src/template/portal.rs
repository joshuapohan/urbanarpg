


use godot::{classes::{Area2D, IArea2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Portal {

    #[base]
    base: Base<Area2D>,

    #[export]
    target_map_id: i32,
}

#[godot_api]
impl Portal{

}

#[godot_api]
impl IArea2D for Portal{
    fn init(base: Base<Area2D>) -> Self {
        Self{
            base: base,
            target_map_id: 0,
        }
    }
}
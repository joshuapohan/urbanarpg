
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct LevelRoot {

    #[base]
    base: Base<Node2D>
}

#[godot_api]
impl LevelRoot{

}

#[godot_api]
impl INode2D for LevelRoot{
    fn init(base: Base<Node2D>) -> Self {
        Self{
            base: base,
        }
    }
}
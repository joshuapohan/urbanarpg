use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};
use godot::classes::{Node2D, INode2D};


#[derive(GodotClass)]
#[class(base=Node2D)]
struct MainNode {

    base: Base<Node2D>
}

#[godot_api]
impl MainNode{

}

#[godot_api]
impl INode2D for MainNode{
    fn init(base: Base<Node2D>) -> Self {
        Self{
            base: base
        }
    }
}
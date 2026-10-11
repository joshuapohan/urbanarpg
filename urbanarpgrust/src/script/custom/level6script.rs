use godot::{classes::{Node2D, INode2D}, prelude::*};


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Level6Script {
    base: Base<Node2D>,    
}

#[godot_api]
impl INode2D for Level6Script {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self,) {
        unimplemented !()
    }

    fn process(&mut self, delta: f64,) {
        unimplemented !()
    }

    fn physics_process(&mut self, delta: f64,) {
        unimplemented !()
    }
}
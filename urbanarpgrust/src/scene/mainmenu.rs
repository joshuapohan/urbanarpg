use godot::prelude::*;
use godot::classes::{Button, CanvasLayer, ColorRect, HBoxContainer, ICanvasLayer, Texture2D, Texture2DArrayRd, TextureRect};


#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct MainMenu {

    base: Base<CanvasLayer>,
    start_button: Option<Gd<Button>>,
    quit_button: Option<Gd<Button>>,

}

#[godot_api]
impl MainMenu{
    #[signal]
    fn s_start_button_pressed();
    #[signal]
    fn s_quit_button_pressed();

    #[func]
    fn on_start_button_pressed(&mut self){
        self.signals().s_start_button_pressed().emit();
    }

    #[func]
    fn on_quit_button_pressed(&mut self){
        self.signals().s_quit_button_pressed().emit();
    }     
}

#[godot_api]
impl ICanvasLayer for MainMenu{
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base,
            start_button: None,
            quit_button: None,
        }
    }

    fn ready(&mut self){
        self.start_button = self.base().get_node_as::<Button>("MenuContainer/StartButton").into();
        self.quit_button = self.base().get_node_as::<Button>("MenuContainer/QuitButton").into();

        let start_button_callable = Callable::from_object_method(&self.base(),"on_start_button_pressed");
        self.start_button.as_mut().unwrap().connect("pressed", &start_button_callable);
        
        let quit_button_callable = Callable::from_object_method(&self.base(),"on_quit_button_pressed");
        self.quit_button.as_mut().unwrap().connect("pressed", &quit_button_callable);

    }
}


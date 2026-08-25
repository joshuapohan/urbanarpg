use godot::classes::{Input, InputEvent, TextureButton};
use godot::classes::{CanvasLayer, ICanvasLayer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Dpad {
    base: Base<CanvasLayer>,

    is_shown: bool,

    move_up_btn: Option<Gd<TextureButton>>,
    move_down_btn: Option<Gd<TextureButton>>,
    move_left_btn: Option<Gd<TextureButton>>,
    move_right_btn: Option<Gd<TextureButton>>,
    attack_btn: Option<Gd<TextureButton>>,
    interact_btn: Option<Gd<TextureButton>>,
    pause_btn: Option<Gd<TextureButton>>,
    toggle_dpad_btn: Option<Gd<TextureButton>>,
}

#[godot_api]
impl Dpad {
    #[func]
    fn on_move_up_btn_down(&mut self) {
        Input::singleton().action_press("move_up");
    }

    #[func]
    fn on_move_up_btn_up(&mut self) {
        Input::singleton().action_release("move_up");
    }

    #[func]
    fn on_move_down_btn_down(&mut self) {
        Input::singleton().action_press("move_down");
    }

    #[func]
    fn on_move_down_btn_up(&mut self) {
        Input::singleton().action_release("move_down");
    }

    #[func]
    fn on_move_left_btn_down(&mut self) {
        Input::singleton().action_press("move_left");
    }

    #[func]
    fn on_move_left_btn_up(&mut self) {
        Input::singleton().action_release("move_left");
    }

    #[func]
    fn on_move_right_btn_down(&mut self) {
        Input::singleton().action_press("move_right");
    }

    #[func]
    fn on_move_right_btn_up(&mut self) {
        Input::singleton().action_release("move_right");
    }

    #[func]
    fn on_attack_btn_down(&mut self) {
        Input::singleton().action_press("attack(physical)");
    }

    #[func]
    fn on_attack_btn_up(&mut self) {
        Input::singleton().action_release("attack(physical)");
    }

    #[func]
    fn on_interact_btn_down(&mut self) {
        Input::singleton().action_press("interact");
    }

    #[func]
    fn on_interact_btn_up(&mut self) {
        Input::singleton().action_release("interact");
    }

    #[func]
    fn on_pause_btn_down(&mut self) {
        Input::singleton().action_press("pause");
    }

    #[func]
    fn on_pause_btn_up(&mut self) {
        Input::singleton().action_release("pause");
    }

    #[func]
    fn on_toggle_dpad_btn_up(&mut self) {
        self.is_shown = !self.is_shown;
        if self.is_shown {
            self.move_up_btn.as_mut().unwrap().show();
            self.move_left_btn.as_mut().unwrap().show();
            self.move_right_btn.as_mut().unwrap().show();
            self.move_down_btn.as_mut().unwrap().show();
            self.interact_btn.as_mut().unwrap().show();
            self.attack_btn.as_mut().unwrap().show();
            self.pause_btn.as_mut().unwrap().show();
        } else {
            self.move_up_btn.as_mut().unwrap().hide();
            self.move_left_btn.as_mut().unwrap().hide();
            self.move_right_btn.as_mut().unwrap().hide();
            self.move_down_btn.as_mut().unwrap().hide();
            self.interact_btn.as_mut().unwrap().hide();
            self.attack_btn.as_mut().unwrap().hide();
            self.pause_btn.as_mut().unwrap().hide();
        }

        self.release_all();
    }    

    #[func]
    fn release_all(&mut self) {
        Input::singleton().action_release("move_up");
        Input::singleton().action_release("move_down");
        Input::singleton().action_release("move_left");
        Input::singleton().action_release("move_right");
        Input::singleton().action_release("attack(physical)");
        Input::singleton().action_release("interact");
        Input::singleton().action_release("pause");
    }    
}

#[godot_api]
impl ICanvasLayer for Dpad {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base, 
            is_shown: false,
            move_up_btn: None, 
            move_down_btn: None, 
            move_left_btn: None, 
            move_right_btn: None, 
            attack_btn: None, 
            interact_btn: None, 
            pause_btn: None,
            toggle_dpad_btn: None,
        }
    }

    fn ready(&mut self){
        self.move_up_btn = self.base().get_node_as::<TextureButton>("Control/movement/up").into();
        self.move_left_btn = self.base().get_node_as::<TextureButton>("Control/movement/left").into();
        self.move_right_btn = self.base().get_node_as::<TextureButton>("Control/movement/right").into();
        self.move_down_btn = self.base().get_node_as::<TextureButton>("Control/movement/down").into();
        self.interact_btn = self.base().get_node_as::<TextureButton>("Control/action/interact").into();
        self.attack_btn = self.base().get_node_as::<TextureButton>("Control/action/attack").into();
        self.toggle_dpad_btn = self.base().get_node_as::<TextureButton>("Control/toggle/show").into();
        self.pause_btn = self.base().get_node_as::<TextureButton>("ControlTop/menu/pause").into();


        // up
        let move_up_btn_down_callable = Callable::from_object_method(&self.base(), "on_move_up_btn_down");
        self.move_up_btn.as_mut().unwrap().connect("button_down", &move_up_btn_down_callable);
        let move_up_btn_up_callable = Callable::from_object_method(&self.base(), "on_move_up_btn_up");
        self.move_up_btn.as_mut().unwrap().connect("button_up", &move_up_btn_up_callable);


        // down
        let move_down_btn_down_callable = Callable::from_object_method(&self.base(), "on_move_down_btn_down");
        self.move_down_btn.as_mut().unwrap().connect("button_down", &move_down_btn_down_callable);
        let move_down_btn_up_callable = Callable::from_object_method(&self.base(), "on_move_down_btn_up");
        self.move_down_btn.as_mut().unwrap().connect("button_up", &move_down_btn_up_callable);


        // left
        let move_left_btn_down_callable = Callable::from_object_method(&self.base(), "on_move_left_btn_down");
        self.move_left_btn.as_mut().unwrap().connect("button_down", &move_left_btn_down_callable);
        let move_left_btn_up_callable = Callable::from_object_method(&self.base(), "on_move_left_btn_up");
        self.move_left_btn.as_mut().unwrap().connect("button_up", &move_left_btn_up_callable);


        // right
        let move_right_btn_down_callable = Callable::from_object_method(&self.base(), "on_move_right_btn_down");
        self.move_right_btn.as_mut().unwrap().connect("button_down", &move_right_btn_down_callable);
        let move_right_btn_up_callable = Callable::from_object_method(&self.base(), "on_move_right_btn_up");
        self.move_right_btn.as_mut().unwrap().connect("button_up", &move_right_btn_up_callable);


        // attack
        let attack_btn_down_callable = Callable::from_object_method(&self.base(), "on_attack_btn_down");
        self.attack_btn.as_mut().unwrap().connect("button_down", &attack_btn_down_callable);
        let attack_btn_up_callable = Callable::from_object_method(&self.base(), "on_attack_btn_up");
        self.attack_btn.as_mut().unwrap().connect("button_up", &attack_btn_up_callable);

        // interact
        let interact_btn_down_callable = Callable::from_object_method(&self.base(), "on_interact_btn_down");
        self.interact_btn.as_mut().unwrap().connect("button_down", &interact_btn_down_callable);
        let interact_btn_up_callable = Callable::from_object_method(&self.base(), "on_interact_btn_up");
        self.interact_btn.as_mut().unwrap().connect("button_up", &interact_btn_up_callable);

        // pause
        let pause_btn_down_callable = Callable::from_object_method(&self.base(), "on_pause_btn_down");
        self.pause_btn.as_mut().unwrap().connect("button_down", &pause_btn_down_callable);
        let pause_btn_up_callable = Callable::from_object_method(&self.base(), "on_pause_btn_up");
        self.pause_btn.as_mut().unwrap().connect("button_up", &pause_btn_up_callable);

        // toggle dpad
        let toggle_dpad_btn_down_callable = Callable::from_object_method(&self.base(), "on_toggle_dpad_btn_up");
        self.toggle_dpad_btn.as_mut().unwrap().connect("button_down", &toggle_dpad_btn_down_callable);

    }
}

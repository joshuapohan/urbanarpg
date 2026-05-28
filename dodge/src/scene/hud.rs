use godot::classes::tween::{EaseType, TransitionType};
use godot::prelude::*;
use godot::classes::{CanvasLayer, ColorRect, ICanvasLayer};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct HUD {
    #[base]
    base: Base<CanvasLayer>,

    fade_overlay: Option<Gd<ColorRect>>
}

#[godot_api]
impl HUD {
    #[signal]
    fn s_fade_in_complete();


    #[signal]
    fn s_fade_out_complete();    

    #[func]
    fn on_fade_in_finished(&mut self){
        self.signals().s_fade_in_complete().emit();
    }

        #[func]
    fn on_fade_out_finished(&mut self){
        self.signals().s_fade_out_complete().emit();
    }

    #[func]
    pub fn fade(&mut self, target: f64){
        godot_print!("fade called target: {}", target);
        let current_alpha = self.fade_overlay.as_ref().unwrap().get_modulate().a;
        godot_print!("current alpha: {}", current_alpha);        
        
        let fade_overlay = self.fade_overlay.clone().unwrap().upcast::<Object>();
        let mut tween = self.base_mut().create_tween();
        tween.tween_property(
            &fade_overlay,
            "modulate:a",
            &target.to_variant(),
            2.0,
        );

        let signal_name = if target == 0.0 {
            "on_fade_in_finished"
        } else {
            "on_fade_out_finished"
        };
        let callable = Callable::from_object_method(&self.base(), signal_name);
        tween.connect("finished", &callable);        
    }
}

#[godot_api]
impl ICanvasLayer for HUD{
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base,
            fade_overlay: None,
        }
    }

    
    fn ready(&mut self){
        self.fade_overlay = self.base().get_node_as::<ColorRect>("FadeOverlay").into();


    }
}
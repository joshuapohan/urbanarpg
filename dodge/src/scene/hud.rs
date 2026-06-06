use godot::classes::tween::{EaseType, TransitionType};
use godot::prelude::*;
use godot::classes::{CanvasLayer, ColorRect, HBoxContainer, ICanvasLayer, Texture2D, Texture2DArrayRd, TextureRect};


const HEART_SIZE: i32 = 20;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct HUD {
    #[base]
    base: Base<CanvasLayer>,

    fade_overlay: Option<Gd<ColorRect>>,
    hearts_container: Option<Gd<HBoxContainer>>,

    hearts_full_texture: Option<Gd<Texture2D>>,
    hearts_half_texture: Option<Gd<Texture2D>>,
    hearts_empty_texture: Option<Gd<Texture2D>>,
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

    #[func]
    fn update_health(&mut self, new_health: i32){

        let hearts = self.hearts_container.as_ref().unwrap().get_children();
        let max_hearts = hearts.len() as i32;
        let full_hearts = new_health / HEART_SIZE;
        let half_hearts = if new_health % HEART_SIZE > 0 {
            1
        } else {
            0
        };
        let empty_hearts = max_hearts - full_hearts - half_hearts;
        let mut heart_index = 0;
        for _ in 0..full_hearts {
            let mut heart_img = hearts.at(heart_index).cast::<TextureRect>();
            heart_img.set_texture(self.hearts_full_texture.as_ref());
            heart_index += 1;
        }
        if half_hearts > 0 {
            let mut heart_img = hearts.at(heart_index).cast::<TextureRect>();
            heart_img.set_texture(self.hearts_half_texture.as_ref());
            heart_index += 1;                   
        }
        for _ in 0..empty_hearts {
            let mut heart_img = hearts.at(heart_index).cast::<TextureRect>();
            heart_img.set_texture(self.hearts_empty_texture.as_ref());
            heart_index += 1;            
        }
    }
}

#[godot_api]
impl ICanvasLayer for HUD{
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base,
            fade_overlay: None,
            hearts_container: None,
            hearts_empty_texture: None,
            hearts_full_texture: None,
            hearts_half_texture: None,
        }
    }

    
    fn ready(&mut self){
        self.fade_overlay = self.base().get_node_as::<ColorRect>("FadeOverlay").into();
        self.hearts_container = self.base().get_node_as::<HBoxContainer>("HeartsContainer").into();

        self.hearts_empty_texture = Some(load::<Texture2D>("res://assets/images/ui/heart_empty.png"));
        self.hearts_half_texture = Some(load::<Texture2D>("res://assets/images/ui/heart_half.png"));
        self.hearts_full_texture = Some(load::<Texture2D>("res://assets/images/ui/heart_full.png"));

    }
}
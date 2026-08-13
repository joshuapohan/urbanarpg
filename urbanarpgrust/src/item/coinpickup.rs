use godot::classes::{Area2D, AudioStreamPlayer2D, IArea2D};
use godot::prelude::*;
use godot::register::GodotClass;

use crate::entity::adventurer::Adventurer;
use crate::item::basepickup::BasePickup;
use crate::log_info;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct CoinPickup {
    base: Base<Area2D>,
    base_pickup: BasePickup,
}

#[godot_api]
impl CoinPickup {
    #[func]
    fn on_pickup_audio_finished(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.get_name().contains("Adventurer") {
            if let Ok(mut adventurer) = body.try_cast::<Adventurer>() {
                let mut bind_adventurer = adventurer.bind_mut();
                self.base_pickup.on_pickup(self.to_gd().upcast::<Area2D>());
            }
        }
    }
}

#[godot_api]
impl IArea2D for CoinPickup {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            base_pickup: BasePickup { pickup_audio: None },
        }
    }
}

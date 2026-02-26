use godot::classes::Label;
use godot::classes::Marker2D;
use godot::classes::PathFollow2D;
use godot::classes::RigidBody2D;
use godot::classes::Timer;
use godot::prelude::*;
use godot::classes::Node;
use godot::classes::INode;
use crate::entity::player::Player;
use rand::Rng;



#[derive(GodotClass)]
#[class(base=Node)]
struct Game {

    #[base]
    base: Base<Node>,

    mob_timer: Option<Gd<Timer>>,
    start_timer: Option<Gd<Timer>>,
    score_timer: Option<Gd<Timer>>,
    starting_position: Option<Gd<Marker2D>>,
    player: Option<Gd<Player>>,
    mob_spawn_location: Option<Gd<PathFollow2D>>,

    #[export]
    mob_scene: Option<Gd<PackedScene>>,    

    score_label: Option<Gd<Label>>,

    score: u64,
}

#[godot_api]
impl Game{

    fn game_over(&mut self){
        self.score_timer.as_mut().unwrap().stop();
        self.mob_timer.as_mut().unwrap().stop();
    }

    fn new_game(&mut self){
        godot_print!("new game");
        let start_position_vector2 = self.starting_position.as_ref().unwrap().get_position();
        self.player.as_mut().unwrap().set_position(start_position_vector2);
        self.start_timer.as_mut().unwrap().start();
        self.player.as_mut().unwrap().bind_mut().start(start_position_vector2);
        self.score_timer.as_mut().unwrap().start();
        godot_print!("timer started");
    }

    #[func]
    fn on_start_timer_timeout(&mut self){
        godot_print!("Start timer timeout");
        self.mob_timer.as_mut().unwrap().start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self){
        godot_print!("Mob timer timeout");

        let mut mob: Gd<RigidBody2D> = self.mob_scene.as_ref().unwrap().instantiate_as::<RigidBody2D>();

        let mob_location = self.mob_spawn_location.as_mut().unwrap();
        mob_location.set_progress_ratio(rand::thread_rng().gen_range(0.0..=1.0_f32));

        let mob_position = mob_location.get_position();

        mob.set_position(mob_position);

        let direction = self.mob_spawn_location.as_ref().unwrap().get_rotation() + std::f32::consts::PI / 2.0
            + rand::thread_rng().gen_range(-std::f32::consts::PI / 4.0..=std::f32::consts::PI / 4.0);

        mob.set_rotation(direction);
        mob.set_linear_velocity(Vector2::new(rand::thread_rng().gen_range(150.0..=250.0), 0.0).rotated(direction));

        self.base_mut().add_child(&mob);
    }

    #[func]
    fn on_score_timer_timeout(&mut self){
        godot_print!("Score timer timeout");
        self.score += 1;
        self.score_label.as_mut().unwrap().set_text(&self.score.to_string());
    }

    #[func]
    fn on_player_hit(&mut self){
        self.game_over();
    }

}

#[godot_api]
impl INode for Game{
    fn init(base: Base<Node>) -> Self {
        godot_print!("Game Initialized");
        Self {base, mob_timer: None, start_timer: None, score_timer: None, 
            starting_position: None, mob_spawn_location: None, player: None , score: 0,
            mob_scene: None, score_label: None,
        }
    }

    fn ready(&mut self){
        godot_print!("Starting Game");
        self.start_timer = self.base().get_node_as::<Timer>("StartTimer").into();
        self.mob_timer = self.base().get_node_as::<Timer>("MobTimer").into();
        self.score_timer = self.base().get_node_as::<Timer>("ScoreTimer").into();
        self.starting_position = self.base().get_node_as::<Marker2D>("StartPosition").into();
        self.player = self.base().get_node_as::<Player>("Player").into();
        self.mob_spawn_location = self.base().get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation").into();
        self.score_label = self.base().get_node_as::<Label>("ScoreLabel").into();        

        // Initialize signals
        let start_timer_callable = Callable::from_object_method(&self.base(), "on_start_timer_timeout");
        if let Some(start_timer) = &mut self.start_timer{
            start_timer.connect("timeout", &start_timer_callable);
        }
        let mob_timer_callable = Callable::from_object_method(&self.base(), "on_mob_timer_timeout");
        if let Some(mob_timer) = &mut self.mob_timer{
            mob_timer.connect("timeout", &mob_timer_callable);
        }
        let score_timer_callable = Callable::from_object_method(&self.base(), "on_score_timer_timeout");
        if let Some(score_timer) = &mut self.score_timer{
            score_timer.connect("timeout", &score_timer_callable);
        }
        let player_hit_callable = Callable::from_object_method(&self.base(), "on_player_hit");
        if let Some(player) = &mut self.player{
            player.connect("hit", &player_hit_callable);
        }

        self.new_game();
    }

}
use godot::prelude::*;

#[derive(PartialEq, Debug)]
enum EGameStates {
    INITIAL,
    RUNNING,
    PAUSEGAMEPLAY,
    PAUSEALL,
    MENU,
}

#[derive(PartialEq, Debug)]
enum EGameStateContext {
    MAINMENU,
    LEVEL(i32),
    ENDSCREEN,
    DIALOGUE,
}

#[derive(GodotClass)]
#[class(singleton)]
pub struct GameState{
    base: Base<Object>,
    current_state: EGameStates,
    current_context :EGameStateContext,
    current_level: i32,
}


#[godot_api]
impl GameState {
    #[func]
    pub fn set_game_context_level(&mut self, id: i32){
        self.current_context = EGameStateContext::LEVEL(id);
        self.current_level = id;
    }

    #[func]
    pub fn set_game_context_mainmenu(&mut self){
        self.current_context = EGameStateContext::MAINMENU;
    }

    #[func]
    pub fn set_game_context_endgame(&mut self){
        self.current_context = EGameStateContext::ENDSCREEN;
    }
    
    #[func]
    pub fn set_game_context_dialogue(&mut self){
        self.current_context = EGameStateContext::DIALOGUE;
    }

    #[func]
    pub fn pause_gameplay(&mut self){
        self.current_state = EGameStates::PAUSEGAMEPLAY;
    }

    #[func]
    pub fn resume_gameplay(&mut self){
        self.current_state = EGameStates::RUNNING;
    }    

    #[func]
    pub fn is_gameplay_paused(&self) -> bool{
        self.current_state == EGameStates::PAUSEGAMEPLAY
    }
    
    #[func]
    pub fn is_in_dialogue(&self) -> bool{
        self.current_context == EGameStateContext::DIALOGUE
    }        
}

#[godot_api]
impl IObject for GameState {
    fn init(base: Base<Object>) -> Self {
       Self {
        base,
        current_state: EGameStates::INITIAL,
        current_context: EGameStateContext::MAINMENU,
        current_level: 0,
       }
    }

}

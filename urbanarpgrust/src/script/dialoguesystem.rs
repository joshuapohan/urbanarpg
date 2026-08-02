use crate::script::{dialoguenode::{DialogueChoice,DialogueNode, load_dialogues}, eventtracker::EventTracker, gamestate};
use godot::prelude::*;
use std::collections::HashMap;
use crate::log_info;
use crate::log_error;

#[derive(GodotClass)]
#[class(singleton)]
pub struct DialogueSystem {
    base: Base<Object>,
    dialogues: HashMap<GString, DialogueNode>,
    current_dialogue_node: Option<DialogueNode>,
    current_line_index: i32,

    is_in_dialogue_choice: bool,
    current_choices: Option<Vec<DialogueChoice>>,
    current_choice_index: i32,
}

#[godot_api]
impl DialogueSystem {
    #[signal]
    fn s_dialogue_started(line: GString, speaker: GString, avatar_id: GString);

    #[signal]
    fn s_skip_typewriter();

    #[signal]
    fn s_dialogue_ended();    

    #[signal]
    fn s_choice_highlighted(index: i32);

    #[signal]
    fn s_choices_shown(choices: Vec<GString>);

    #[signal]
    fn s_dialogue_event(event_id: GString);

    #[func]
    pub fn start_dialogue_by_id(&mut self, dialogue_id: GString) {
        self.is_in_dialogue_choice = false;
        self.current_choice_index = 0;
        self.current_choices = None;
        
        let dialogue_node = self.dialogues.get(&dialogue_id);
        if dialogue_node.is_some() {
            self.current_dialogue_node = Some(dialogue_node.unwrap().clone());
            self.current_line_index = 0;
            self.show_current_line();
        } else {
            log_info!("Dialogue id not found {}", dialogue_id);
            self.signals().s_dialogue_ended().emit();
        }
    }

    #[func]
    pub fn advance(&mut self){
        let is_dialogue_end = gamestate::GameState::singleton().bind().is_dialogue_state_end();
        if  !is_dialogue_end {
            log_info!("Skip typewriter");
            self.signals().s_skip_typewriter().emit();            
        } else {
            log_info!("Next line");
            self.next_line();
        }
    }

    #[func]
    pub fn move_choice_up(&mut self){
        log_info!("In dialogue, up 1");
        if self.is_in_dialogue_choice && self.current_choices.is_some(){
            log_info!("In dialogue, up 2");

            let choices = self.current_choices.as_ref().unwrap();
            if self.current_choice_index <= 0 {
                self.current_choice_index = choices.len() as i32 - 1;
            } else {
                self.current_choice_index = self.current_choice_index - 1;
            }
            let index = self.current_choice_index;            
            self.signals().s_choice_highlighted().emit( index);
            log_info!("In dialogue, up 3 {}", self.current_choice_index);
        }
    }

    #[func]
    pub fn move_choice_down(&mut self){
        log_info!("In dialogue, down 1");
        if self.is_in_dialogue_choice && self.current_choices.is_some(){
            log_info!("In dialogue, down 2");
            let choices = self.current_choices.as_ref().unwrap();
            if self.current_choice_index >= choices.len() as i32 - 1 {
                self.current_choice_index = 0
            } else {
                self.current_choice_index = self.current_choice_index + 1;
            }
            let index = self.current_choice_index;            
            self.signals().s_choice_highlighted().emit(index);
            log_info!("In dialogue, down 3 {}", self.current_choice_index);
        }
    }
    
    #[func]
    pub fn select_current_choice(&mut self){
        if self.is_in_dialogue_choice && self.current_choices.is_some(){
            let choices = self.current_choices.as_ref().unwrap();
            if self.current_choice_index >= choices.len() as i32 {
                log_error!("Invalid dialogue choice index {}", self.current_choice_index);
                self.signals().s_dialogue_ended().emit();
                return
            }
            let index = self.current_choice_index as usize;
            let choice_node = choices.get(index);
            if choice_node.is_none() {
                log_error!("None choice node {}", self.current_choice_index);
                self.signals().s_dialogue_ended().emit();
                return
            } else {
                let next_dialogue_id = choice_node.as_ref().unwrap().next_id.clone();
                log_info!("Starting next dialogue id: {}", next_dialogue_id);
                self.start_dialogue_by_id(next_dialogue_id);
            }
        }
    }       


    #[func]
    pub fn next_line(&mut self) {

        if self.current_dialogue_node.is_none() {
            log_info!("Showing current line on nonexistent dialogue node");
            self.signals().s_dialogue_ended().emit();
            return;
        }

        if self.is_in_dialogue_choice {
            self.select_current_choice();
            return;
        }

        self.current_line_index = self.current_line_index + 1;
        let node = self.current_dialogue_node.as_ref().unwrap();
        log_info!("current line index {} , dialogue len {}",self.current_line_index, node.lines.len() );
        if self.current_line_index >= node.lines.len() as i32 {
            match &node.choices {
                Some(choices) => {
                    let mut choices_list = Vec::<GString>::new();
                    for choice in choices {
                        choices_list.push(choice.text.clone());
                    }
                    self.is_in_dialogue_choice = true;
                    self.current_choice_index = 0;
                    self.current_choices = node.choices.clone();
                    self.signals().s_choices_shown().emit(choices_list);
                }
                None => self.end_dialogue(),
            }
        } else {
            self.show_current_line();
        }
    }

    #[func]
    fn show_current_line(&mut self) {
        if self.current_dialogue_node.is_none() {
            log_info!("Showing current line on nonexistent dialogue node");
            self.signals().s_dialogue_ended().emit();
            return;
        }
        
        if self.current_line_index > self.current_dialogue_node.as_ref().unwrap().lines.len() as i32{
            log_info!("line index exceeds dialogue lines");
            self.signals().s_dialogue_ended().emit();
            return;
        }

        let (text, speaker, avatar) = {
            let line = self.current_dialogue_node.as_ref().unwrap().lines.get(self.current_line_index as usize);
            if line.is_some() {
                (line.unwrap().text.clone(), line.unwrap().speaker_name.clone(), line.unwrap().avatar_id.clone())
            } else {
                ("".into(), "".into(), "".into())
            }
        };
        
        log_info!("Dialogue : {} {} : {}", avatar, speaker, text);

        self.signals().s_dialogue_started().emit(
            &text,
            &speaker,
            &avatar,
        );
    }

    #[func]
    fn end_dialogue(&mut self) {
        if self.current_dialogue_node.is_none() {
            log_info!("Ending nonexistent dialogue");
            self.signals().s_dialogue_ended().emit();
            return;
        }
        let current_node = self.current_dialogue_node.take();
        if current_node.is_some() {
            if current_node.as_ref().unwrap().on_exit_event.is_some(){
                let event_id = current_node.as_ref().unwrap().on_exit_event.as_ref().unwrap().clone();
                EventTracker::singleton().bind_mut().trigger_event(event_id);
            }
        }
        log_info!("ending current dialogue");
        self.signals().s_dialogue_ended().emit();
    }
}

#[godot_api]
impl IObject for DialogueSystem {
    fn init(base: Base<Object>) -> Self {
        let dialogues = load_dialogues();

        Self {
            base,
            dialogues: dialogues,
            current_dialogue_node: None,
            current_line_index: 0,
            is_in_dialogue_choice: false,
            current_choices: None,
            current_choice_index: 0
        }
    }
}

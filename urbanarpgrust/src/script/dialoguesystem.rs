use crate::script::dialoguenode::{DialogueLine, DialogueNode};
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(singleton)]
pub struct DialogueSystem {
    base: Base<Object>,
    dialogues: HashMap<GString, DialogueNode>,
    current_dialogue_node: Option<DialogueNode>,
    current_line_index: i32,
}

#[godot_api]
impl DialogueSystem {
    #[signal]
    fn s_dialogue_started(line: GString, speaker: GString, avatar_id: GString);

    #[signal]
    fn s_dialogue_ended();

    #[signal]
    fn s_choices_shown(choices: Vec<GString>);

    #[signal]
    fn s_dialogue_event(event_id: GString);

    #[func]
    pub fn start_dialogue_by_id(&mut self, dialogue_id: GString) {
        let dialogue_node = self.dialogues.get(&dialogue_id);
        if dialogue_node.is_some() {
            self.current_dialogue_node = Some(dialogue_node.unwrap().clone());
            self.current_line_index = 0;
            self.show_current_line();
        } else {
            godot_print!("Dialogue id not found {}", dialogue_id);
        }
    }

    #[func]
    pub fn next_line(&mut self) {
        if self.current_dialogue_node.is_none() {
            godot_print!("Showing current line on nonexistent dialogue node");
            return;
        }
        self.current_line_index = self.current_line_index + 1;
        let node = self.current_dialogue_node.as_ref().unwrap();
        godot_print!("current line index {} , dialogue len {}",self.current_line_index, node.lines.len() );
        if self.current_line_index >= node.lines.len() as i32 {
            match &node.choices {
                Some(choices) => {
                    let mut choices_list = Vec::<GString>::new();
                    for choice in choices {
                        choices_list.push(choice.text.clone());
                    }
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
            godot_print!("Showing current line on nonexistent dialogue node");
            return;
        }
        
        if self.current_line_index > self.current_dialogue_node.as_ref().unwrap().lines.len() as i32{
            godot_print!("line index exceeds dialogue lines");
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
        
        godot_print!("Dialogue : {} {} : {}", avatar, speaker, text);

        self.signals().s_dialogue_started().emit(
            &text,
            &speaker,
            &avatar,
        );
    }

    #[func]
    pub fn on_choice_selected(&mut self, index: i32) {
        if self.current_dialogue_node.is_none() {
            godot_print!("Choice selection on nonexistent dialogue node");
            return;
        }
        if self
            .current_dialogue_node
            .as_ref()
            .unwrap()
            .choices
            .is_none()
        {
            godot_print!("Choice selection on node with no choices");
            return;
        }
        if (self
            .current_dialogue_node
            .as_ref()
            .unwrap()
            .choices
            .as_ref()
            .unwrap()
            .len() as i32)  < index {
            godot_print!("Choice selection on out of range choices index {}", index);
            return;
        }
        let next_id = self
            .current_dialogue_node
            .as_ref()
            .unwrap()
            .choices
            .as_ref()
            .unwrap().get(index as usize)
            .as_ref().unwrap()
            .next_id.clone();

        self.start_dialogue_by_id(next_id);
    }

    #[func]
    fn end_dialogue(&mut self) {
        if self.current_dialogue_node.is_none() {
            godot_print!("Ending nonexistent dialogue");
            return;
        }
        let current_node = self.current_dialogue_node.take();
        if current_node.is_some() {
            if current_node.as_ref().unwrap().on_exit_event.is_none(){

            } else {
                let event_id = current_node.as_ref().unwrap().on_exit_event.as_ref().unwrap().clone();
                self.signals()
                    .s_dialogue_event()
                    .emit(&event_id);
            }
        }
        godot_print!("ending current dialogue");
        self.signals().s_dialogue_ended().emit();
    }
}

#[godot_api]
impl IObject for DialogueSystem {
    fn init(base: Base<Object>) -> Self {
        let testnode = DialogueNode{ 
            id: "123".into(), 
            lines: vec![
                DialogueLine{ 
                    text: "Hello".into(), 
                    speaker_name: "Speaker".into(), 
                    avatar_id: "hero".into() 
                }
            ], 
            choices: None, 
            on_exit_event: None 
        };
        let mut dialogues = HashMap::<GString, DialogueNode>::new();
        dialogues.insert("123".into(), testnode);
        Self {
            base,
            dialogues: dialogues,
            current_dialogue_node: None,
            current_line_index: 0,
        }
    }
}

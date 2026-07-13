use crate::script::DialogueNode;
use godot::prelude::*;
use std::collections::Hashmap;

#[derive(GodotClass)]
#[class(singleton)]
pub struct DialogueSystem {
    base: Base<Object>,
    dialogues: Hashmap<GString, DialogueNode>,
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
    fn start_dialogue_by_id(dialogue_id: GString) {
        let dialogue_node = self.dialogues.get(dialogue_id);
        if dialogue_node.is_some() {
            self.current_dialogue_node = Some(dialogue_node);
            self.show_current_line();
        } else {
            godot_print!("Dialogue id not found {}", dialogue_id);
        }
    }

    #[func]
    fn next_line(&mut self) {
        if self.current_dialogue_node.is_none() {
            godot_print("Showing current line on nonexistent dialogue node");
            return;
        }
        self.current_line_index = self.current_line_index + 1;
        let node = self.current_dialogue_node.as_ref().unwrap();
        if self.current_line_index > &node.lines.len() {
            match &node.choices {
                Some(choices) => {
                    let choices_list = Vec::new<GString>();
                    for choice in choices {
                        choices_list.push(choice.text.clone());
                    }
                    self.signals().s_choices_shown(choices_list).emit();
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
            godot_print("Showing current line on nonexistent dialogue node");
            return;
        }
        let node = self.current_dialogue_node.as_ref().unwrap();
        if self.current_line_index > &node.lines.len() {
            godot_print("line index exceeds dialogue lines");
            return;
        }
        let line = &node.lines[self.current_line_index];
        self.signals().s_dialogue_started.emit(
            line.text.clone(),
            line.speaker_name.clone(),
            line.avatar_id.clone(),
        );
    }

    #[func]
    fn on_choice_selected(&mut self, index: i32) {
        if self.current_dialogue_node.is_none() {
            godot_print("Choice selection on nonexistent dialogue node");
            return;
        }
        if self
            .current_dialogue_node
            .as_ref()
            .unwrap()
            .choices
            .is_none()
        {
            godot_print("Choice selection on node with no choices");
            return;
        }
        if self
            .current_dialogue_node
            .as_ref()
            .unwrap()
            .choices
            .as_ref()
            .unwrap()
            .len()
            < index
        {
            godot_print("Choice selection on out of range choices index {}", index);
            return;
        }
        let next_id = self
            .current_dialogue_node
            .as_ref()
            .unwrap()
            .choices
            .as_ref()
            .unwrap()[index]
            .next_id;

        self.start_dialogue_by_id(next_id);
    }

    #[func]
    fn end_dialogue(&mut self) {
        if self.current_dialogue_node.is_none() {
            godot_print!("Ending nonexistent dialogue");
            return;
        }
        let current_node = self.current_dialogue_node.take();
        if current_node.on_exit_event.is_some() {
            self.signals()
                .s_dialogue_event()
                .emit(current_node.on_exit_event.unwrap().clone());
        }
        self.signals().s_dialogue_ended().emit();
    }
}

#[godot_api]
impl IObject for DialogueSystem {
    #[func]
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            dialogues: Hashmap::new::<GString, DialogueNode>(),
            current_dialogue_node: None,
            current_line_index: 0,
        }
    }
}

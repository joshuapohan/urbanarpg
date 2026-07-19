use std::collections::HashMap;

use godot::{classes::{FileAccess, file_access::ModeFlags}, prelude::*};
use serde::Deserialize;

#[derive(Clone)]
pub struct DialogueLine {
    pub text: GString,
    pub speaker_name: GString,
    pub avatar_id: GString,
}

#[derive(Clone)]
pub struct DialogueChoice {
    pub text: GString,
    pub next_id: GString,
}

#[derive(Clone)]
pub struct DialogueNode{
    pub id: GString,
    pub lines: Vec<DialogueLine>,
    pub choices: Option<Vec<DialogueChoice>>,
    pub on_exit_event: Option<GString>,
}

#[derive(Clone, Deserialize)]
pub struct DialogueLineJSON {
    pub text: String,
    pub speaker_name: String,
    pub avatar_id: String,
}

#[derive(Clone, Deserialize)]
pub struct DialogueChoiceJSON {
    pub text: String,
    pub next_id: String,
}

#[derive(Clone, Deserialize)]
pub struct DialogueNodeJSON{
    pub id: String,
    pub lines: Vec<DialogueLineJSON>,
    pub choices: Option<Vec<DialogueChoiceJSON>>,
    pub on_exit_event: Option<String>,
}

impl From<DialogueLineJSON> for DialogueLine {
    fn from(value: DialogueLineJSON) -> Self {
        DialogueLine { 
            text: value.text.to_gstring(), 
            speaker_name: value.speaker_name.to_gstring(), 
            avatar_id: value.avatar_id.to_gstring(), 
        }
    }
}

impl From<DialogueChoiceJSON> for DialogueChoice {
    fn from(value: DialogueChoiceJSON) -> Self {
        DialogueChoice { 
            text: value.text.to_gstring(), 
            next_id: value.next_id.to_gstring() 
        }
    }
}

impl From<DialogueNodeJSON> for DialogueNode {
    fn from(value: DialogueNodeJSON) -> Self {
        DialogueNode { 
            id: value.id.to_gstring(), 
            lines: value.lines.into_iter().map(|line| line.into()).collect(), 
            choices: value.choices.map(|choice_nodes| {
                choice_nodes.into_iter().map(|choice| choice.into()).collect()
            }), 
            on_exit_event: value.on_exit_event.map(|val| val.to_gstring()),
        }
    }
}

pub fn load_dialogues() -> HashMap<GString, DialogueNode>{
    let mut dialogues = HashMap::<GString, DialogueNode>::new();
    let mut file = FileAccess::open("res://data/dialogues.json", ModeFlags::READ).unwrap();
    let content = file.get_as_text();
    let parsed_dialogue_node_json: HashMap<String, DialogueNodeJSON> = serde_json::from_str(&content.to_string()).unwrap();
    for (key, node) in parsed_dialogue_node_json {
        dialogues.insert(key.to_gstring(), node.into());
    }
    dialogues
}
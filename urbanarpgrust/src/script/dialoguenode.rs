use godot::prelude::*;

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

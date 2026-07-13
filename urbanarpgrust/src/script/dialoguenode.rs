use godot::prelude::*;

pub struct DialogueLine {
    text: GString,
    speaker_name: GString,
    avatar_id: GString,
}

pub struct DialogueChoice {
    text: GString,
    next_id: GString,
}

pub struct DialogueNode{
    id: GString,
    lines: Vec<DialogueLine>,
    choices: Option<Vec<DialogueChoice>>,
    on_exit_event: Option<GString>,
}

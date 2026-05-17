use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod entity;
mod scene;
mod ui;
mod template;
mod script;
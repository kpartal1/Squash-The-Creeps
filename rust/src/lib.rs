use godot::prelude::*;
mod main_node;
mod mob;
mod mobdetector;
mod mobtimer;
mod mobvisiblenotifier;
mod player;
mod scorelabel;

struct SquashTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for SquashTheCreeps {}

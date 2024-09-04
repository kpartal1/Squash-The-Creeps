use godot::{
    engine::{ILabel, Label},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Label)]
pub struct ScoreLabel {
    score: u32,

    base: Base<Label>,
}

#[godot_api]
impl ScoreLabel {
    #[func] // Called by Mob
    fn on_mob_squashed(&mut self) {
        self.score += 1;
        let score = self.score;
        self.base_mut().set_text(format!("Score: {}", score).into());
    }
}

#[godot_api]
impl ILabel for ScoreLabel {
    fn init(base: Base<Label>) -> Self {
        Self { score: 0, base }
    }
}

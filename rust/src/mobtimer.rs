use godot::{
    engine::{ColorRect, ITimer, Timer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Timer)]
pub struct MobTimer {
    base: Base<Timer>,
}

#[godot_api]
impl MobTimer {
    #[func] // Called by Player
    fn on_player_hit(&mut self) {
        self.base_mut().stop();
        self.base()
            .get_parent()
            .unwrap()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .show();
    }
}

#[godot_api]
impl ITimer for MobTimer {
    fn init(base: Base<Timer>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let callable = self
            .base()
            .get_parent()
            .unwrap()
            .callable("on_mob_timer_timeout");
        self.base_mut().connect("timeout".into(), callable);
    }
}

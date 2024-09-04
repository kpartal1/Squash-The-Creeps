use std::f32::consts::FRAC_PI_4;

use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::engine::AnimationPlayer;
use godot::prelude::*;
use rand::Rng;

use crate::scorelabel::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Mob {
    #[export]
    min_speed: i32,
    #[export]
    max_speed: i32,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl Mob {
    #[signal]
    fn squashed();

    #[func]
    pub fn squash(&mut self) {
        let mut base = self.base_mut();
        base.emit_signal("squashed".into(), &[]);
        base.queue_free();
    }

    #[func]
    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        let min_speed = self.min_speed;
        let max_speed = self.max_speed;
        let mut base = self.base_mut();
        base.look_at_from_position(start_position, player_position);

        let mut rng = rand::thread_rng();
        base.rotate_y(rng.gen_range(-FRAC_PI_4..FRAC_PI_4));

        let random_speed = rng.gen_range(min_speed..max_speed);

        base.set_velocity(Vector3::FORWARD * random_speed as f32);

        let velocity = base.get_velocity();
        let rotation = base.get_rotation();

        base.set_velocity(velocity.rotated(Vector3::UP, rotation.y));

        base.get_node_as::<AnimationPlayer>("AnimationPlayer")
            .set_speed_scale((random_speed / min_speed) as f32);
    }

    #[func] // Called by MobVisibleNotifier
    fn screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            min_speed: 10,
            max_speed: 18,
            base,
        }
    }

    fn ready(&mut self) {
        let callable = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<ScoreLabel>("UserInterface/ScoreLabel")
            .callable("on_mob_squashed");
        self.base_mut().connect("squashed".into(), callable);
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

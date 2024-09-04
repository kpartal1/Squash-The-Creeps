use std::f32::consts::PI;

use crate::mob::*;
use crate::mobtimer::*;
use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::engine::AnimationPlayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    #[export]
    speed: f64,
    #[export]
    fall_acceleration: f64,
    target_velocity: Vector3,
    #[export]
    jump_impulse: f64,
    #[export]
    bounce_impulse: f64,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func] // Called by MobDetector
    fn die(&mut self, _: Gd<Mob>) {
        self.base_mut().emit_signal("hit".into(), &[]);
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 14.,
            fall_acceleration: 75.,
            target_velocity: Vector3::ZERO,
            jump_impulse: 20.,
            bounce_impulse: 16.,
            base,
        }
    }

    fn ready(&mut self) {
        let callable = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<MobTimer>("MobTimer")
            .callable("on_player_hit");
        self.base_mut().connect("hit".into(), callable);
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;

        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            direction.x += 1.;
        }
        if input.is_action_pressed("move_left".into()) {
            direction.x -= 1.;
        }
        if input.is_action_pressed("move_back".into()) {
            direction.z += 1.;
        }
        if input.is_action_pressed("move_forward".into()) {
            direction.z -= 1.;
        }

        let mut animation_player = self
            .base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer");
        if direction != Vector3::ZERO {
            direction = direction.normalized();
            self.base_mut()
                .get_node_as::<Node3D>("Pivot")
                .set_basis(Basis::new_looking_at(direction, Vector3::UP, false));
            animation_player.set_speed_scale(4.);
        } else {
            animation_player.set_speed_scale(1.);
        }

        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        if !self.base().is_on_floor() {
            self.target_velocity.y -= (self.fall_acceleration * delta) as f32;
        }

        if self.base().is_on_floor() && input.is_action_just_pressed("jump".into()) {
            self.target_velocity.y = self.jump_impulse as f32;
        }

        for index in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(index).unwrap();

            if let Some(collider) = collision.get_collider() {
                let node = collider.cast::<Node>();
                if node.is_in_group("mob".into()) && Vector3::UP.dot(collision.get_normal()) > 0.1 {
                    node.cast::<Mob>().bind_mut().squash();
                    self.target_velocity.y = self.bounce_impulse as f32;
                    break;
                }
            }
        }

        let velocity = self.target_velocity;
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
        let mut rotation = pivot.get_rotation();
        rotation.x = PI / 6. * velocity.y / self.jump_impulse as f32;
        pivot.set_rotation(rotation);
    }
}

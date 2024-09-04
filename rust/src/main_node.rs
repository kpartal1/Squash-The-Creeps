use crate::mob::*;
use godot::classes::CharacterBody3D;
use godot::classes::PathFollow3D;
use godot::engine::ColorRect;
use godot::engine::InputEvent;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[export]
    mob_scene: Gd<PackedScene>,

    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[func] // Called by MobTimer
    fn on_mob_timer_timeout(&mut self) {
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");

        mob_spawn_location.set_progress_ratio(rand::random());

        let player_position = self
            .base()
            .get_node_as::<CharacterBody3D>("Player")
            .get_position();

        let mut mob = self.mob_scene.instantiate_as::<Mob>();

        mob.bind_mut()
            .initialize(mob_spawn_location.get_position(), player_position);

        self.base_mut().add_child(mob.clone().upcast());
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            mob_scene: PackedScene::new_gd(),
            base,
        }
    }

    fn ready(&mut self) {
        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .hide();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let is_visible = self
            .base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .is_visible();
        if event.is_action_pressed("ui_accept".into()) && is_visible {
            self.base().get_tree().unwrap().reload_current_scene();
        }
    }
}

use godot::classes::VisibleOnScreenNotifier3D;
use godot::engine::IVisibleOnScreenNotifier3D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=VisibleOnScreenNotifier3D)]
struct MobVisibleNotifier {
    base: Base<VisibleOnScreenNotifier3D>,
}

#[godot_api]
impl IVisibleOnScreenNotifier3D for MobVisibleNotifier {
    fn init(base: Base<VisibleOnScreenNotifier3D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let callable = self.base().get_parent().unwrap().callable("screen_exited");
        self.base_mut().connect("screen_exited".into(), callable);
    }
}

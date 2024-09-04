use godot::{
    engine::{Area3D, IArea3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area3D)]
struct MobDetector {
    base: Base<Area3D>,
}

#[godot_api]
impl IArea3D for MobDetector {
    fn init(base: Base<Area3D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let callable = self.base().get_parent().unwrap().callable("die");
        self.base_mut().connect("body_entered".into(), callable);
    }
}

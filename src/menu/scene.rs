use quad::{
    ecs::World,
    input::{KeyCode, KeyboardInput},
    Scene, SceneResult,
};

#[derive(Default)]
pub struct MenuScene {}

impl Scene for MenuScene {
    fn update(&mut self, world: &mut World) -> SceneResult {
        let keyboard = world.resource::<KeyboardInput>();
        if keyboard.just_pressed(KeyCode::Escape) {
            SceneResult::Quit
        } else {
            SceneResult::Ok
        }
    }
}

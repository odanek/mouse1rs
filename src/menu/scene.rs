use quad::{
    ecs::{Res, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    Scene, SceneResult, SceneSchedule,
};

#[derive(Default)]
pub struct MenuScene {}

impl Scene for MenuScene {
    fn run(&mut self, world: &mut World) -> SceneSchedule {
        SceneSchedule {
            update: Some(Scheduler::chain(world).add(&menu_update).build()),
            ..Default::default()
        }
    }
}

fn menu_update(keyboard: Res<KeyboardInput>) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Escape) {
        SceneResult::Quit
    } else {
        SceneResult::Ok
    }
}

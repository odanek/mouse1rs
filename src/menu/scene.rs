use quad::{
    ecs::{Res, Schedule, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    Scene, SceneResult,
};

pub struct MenuScene {
    update: Schedule<(), SceneResult>,
}

impl Scene for MenuScene {
    fn update(&mut self, world: &mut World) -> SceneResult {
        self.update.run(world)
    }
}

impl MenuScene {
    pub fn new(world: &mut World) -> Self {
        Self {
            update: Scheduler::chain(world).add(&menu_update).build(),
        }
    }
}

fn menu_update(keyboard: Res<KeyboardInput>) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Escape) {
        SceneResult::Pop
    } else {
        SceneResult::Ok
    }
}

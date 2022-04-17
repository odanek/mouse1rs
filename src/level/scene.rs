use quad::{
    ecs::{Commands, IntoSystem, Res, Schedule, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    Scene, SceneResult, SceneStage,
};

pub struct LevelSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

pub struct LevelScene {
    schedule: Option<LevelSchedule>,
    level: u32,
}

impl LevelScene {
    pub fn new(level: u32) -> Self {
        Self {
            schedule: None,
            level,
        }
    }
}

impl Scene for LevelScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| LevelSchedule {
            start: Scheduler::single(level_init.system(world)),
            update: Scheduler::single(level_update.system(world)),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn level_init() -> SceneResult {
    SceneResult::Ok
}

fn level_update(mut commands: Commands, keyboard: Res<KeyboardInput>) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Escape) {
        SceneResult::Pop
    } else {
        SceneResult::Ok
    }
}

use quad::{
    ecs::{IntoSystem, Schedule, Scheduler, World},
    run::{Scene, SceneResult, SceneStage},
};

pub struct LostLifeSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

#[derive(Default)]
pub struct LostLifeScene {
    schedule: Option<LostLifeSchedule>,
}

impl Scene for LostLifeScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| LostLifeSchedule {
            start: Scheduler::single(level_opening_start.system(world)),
            update: Scheduler::single(level_opening_update.system(world)),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn level_opening_start() -> SceneResult {
    SceneResult::Ok(SceneStage::Update)
}

fn level_opening_update() -> SceneResult {
    SceneResult::Ok(SceneStage::Update)
}

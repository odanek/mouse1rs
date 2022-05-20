use quad::{
    asset::{AssetServer, Assets},
    ecs::{Commands, IntoSystem, Res, ResMut, Schedule, Scheduler, World},
    render::{texture::Image, AddressMode},
    Scene, SceneResult, SceneStage,
};

use crate::level::{Level, LevelAssets, LevelScene};

pub struct PreloadSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

#[derive(Default)]
pub struct PreloadScene {
    schedule: Option<PreloadSchedule>,
}

impl Scene for PreloadScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| PreloadSchedule {
            start: Scheduler::single(preload_start.system(world)),
            update: Scheduler::single(preload_update.system(world)),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn preload_start(
    mut commands: Commands,
    level: Res<Level>,
    asset_server: Res<AssetServer>,
) -> SceneResult {
    let foreground = asset_server.load(level.fg_path());
    let background = asset_server.load(level.bg_path());

    commands.insert_resource(LevelAssets {
        foreground,
        background,
    });

    SceneResult::Ok(SceneStage::Update)
}

fn preload_update(
    level_assets: Res<LevelAssets>,
    mut images: ResMut<Assets<Image>>,
) -> SceneResult {
    if images.get(&level_assets.foreground).is_some() {
        if let Some(bg_image) = images.get_mut(&level_assets.background) {
            bg_image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
            return SceneResult::Replace(Box::new(LevelScene::default()), SceneStage::Start);
        }
    }

    SceneResult::Ok(SceneStage::Update)
}

use quad::{
    asset::AssetServer,
    ecs::{Commands, IntoSystem, Res, Scheduler, World},
    pipeline::ClearColor,
    render::color::Color,
    Scene, SceneResult,
};

use crate::menu::MenuScene;

use super::GameAssets;

#[derive(Default)]
pub struct MouseScene {}

impl Scene for MouseScene {
    fn update(&mut self, world: &mut World) -> SceneResult {
        Scheduler::single(bootstrap.system(world)).run(world)
    }
}

fn bootstrap(mut commands: Commands, assets: Res<AssetServer>) -> SceneResult {
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.insert_resource(GameAssets {
        font: assets.load("helvetica.ttf"),
    });
    SceneResult::Replace(Box::new(MenuScene::default()))
}

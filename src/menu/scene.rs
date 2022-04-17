use quad::{
    ecs::{Commands, Entity, IntoSystem, Res, ResMut, Resource, Schedule, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    render::color::Color,
    text::{Text, TextBundle, TextSection, TextStyle},
    transform::{Transform, TransformBundle},
    windowing::Windows,
    Scene, SceneResult, SceneStage,
};

use crate::{level::LevelScene, mouse::GameAssets};

struct MenuSceneSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

#[derive(Resource)]
struct MenuData {
    root: Entity,
}

#[derive(Default)]
pub struct MenuScene {
    schedule: Option<MenuSceneSchedule>,
}

impl Scene for MenuScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| MenuSceneSchedule {
            start: Scheduler::single(menu_init.system(world)),
            update: Scheduler::single(menu_update.system(world)),
        });

        match stage {
            SceneStage::Start | SceneStage::Resume => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
        }
    }
}

fn menu_init(
    mut commands: Commands,
    assets: Res<GameAssets>,
    windows: ResMut<Windows>,
) -> SceneResult {
    let window_size = windows.primary().size();

    let menu1 = commands
        .spawn()
        .insert_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "1.  ".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 30.0,
                            color: Color::GREEN,
                        },
                    },
                    TextSection {
                        value: " Nova hra".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 30.0,
                            color: Color::ORANGE_RED,
                        },
                    },
                ],
                ..Default::default()
            },
            transform: Transform::from_xyz(-65.0, 30.0, 0.0),
            ..Default::default()
        })
        .id();

    let menu2 = commands
        .spawn()
        .insert_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "2.  ".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 30.0,
                            color: Color::GREEN,
                        },
                    },
                    TextSection {
                        value: " Konec".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 30.0,
                            color: Color::ORANGE_RED,
                        },
                    },
                ],
                ..Default::default()
            },
            transform: Transform::from_xyz(-65.0, 0.0, 0.0),
            ..Default::default()
        })
        .id();

    let disclaimer = commands
        .spawn()
        .insert_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Napsal O. Danek v roce 2022 v jazyce Rust".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 25.0,
                        color: Color::PINK,
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(
                -window_size.width / 2.0,
                -window_size.height / 2.0 + 25.0,
                0.0,
            ),
            ..Default::default()
        })
        .id();

    let root = commands
        .spawn()
        .push_children(&[menu1, menu2, disclaimer])
        .insert_bundle(TransformBundle::default())
        .id();

    commands.insert_resource(MenuData { root });

    SceneResult::Ok
}

fn menu_update(
    mut commands: Commands,
    data: Res<MenuData>,
    keyboard: Res<KeyboardInput>,
) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Key2) {
        SceneResult::Pop
    } else if keyboard.just_pressed(KeyCode::Key1) {
        commands.entity(data.root).despawn_recursive();
        commands.remove_resource::<MenuData>();
        SceneResult::Push(Box::new(LevelScene::new(0)))
    } else {
        SceneResult::Ok
    }
}

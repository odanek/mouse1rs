use quad::prelude::*;

use crate::{
    level::Level,
    level_opening::LevelOpeningScene,
    mouse::{render_lifes, GameAssets, Lifes},
};

struct MenuSceneSchedule {
    start: Schedule<(), SceneResult>, // TODO Typealias
    update: Schedule<(), SceneResult>,
    pause: Schedule<(), SceneResult>,
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
            start: Scheduler::chain(world)
                .add(render_lifes)
                .add(menu_init)
                .build(),
            update: Scheduler::single(menu_update),
            pause: Scheduler::single(menu_pause),
        });

        match stage {
            SceneStage::Start | SceneStage::Resume => schedule.start.run(world),
            SceneStage::Pause => schedule.pause.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn menu_init(mut commands: Commands, assets: Res<GameAssets>) -> SceneResult {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(UiTextBundle {
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
                        ..Default::default()
                    });

                    parent.spawn().insert_bundle(UiTextBundle {
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
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(UiTextBundle {
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
                        ..Default::default()
                    });
                });
        })
        .id();

    commands.insert_resource(MenuData { root });

    SceneResult::Ok(SceneStage::Update)
}

fn menu_update(keyboard: Res<KeyboardInput>) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Key2) {
        SceneResult::Quit
    } else if keyboard.just_pressed(KeyCode::Key1) {
        SceneResult::Ok(SceneStage::Pause)
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

fn menu_pause(
    mut commands: Commands,
    data: Res<MenuData>,
    mut lifes: ResMut<Lifes>,
) -> SceneResult {
    commands.entity(data.root).despawn_recursive();
    commands.remove_resource::<MenuData>();
    commands.insert_resource(Level(0));
    lifes.count = 5;
    SceneResult::Push(Box::new(LevelOpeningScene::default()), SceneStage::Start)
}

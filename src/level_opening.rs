use quad::{
    ecs::{Commands, Entity, IntoSystem, Res, Resource, Schedule, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    render::color::Color,
    run::{Scene, SceneResult, SceneStage},
    text::{Text, TextSection, TextStyle},
    ty::{Rect, Size},
    ui::{
        entity::{NodeBundle, UiTextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, Val,
    },
};

use crate::{
    level::{Level, LevelScene},
    mouse::GameAssets,
};

pub struct LevelOpeningSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

const LEVEL_NAMES: [&str; 5] = [
    "Horska udoli",
    "Zaplavene jeskyne",
    "Mesto v noci",
    "Tajuplny zamek",
    "Amazonska dzungle",
];

#[derive(Resource)]
struct LevelOpeningData {
    root: Entity,
}

#[derive(Default)]
pub struct LevelOpeningScene {
    schedule: Option<LevelOpeningSchedule>,
}

impl Scene for LevelOpeningScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| LevelOpeningSchedule {
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

fn level_opening_start(
    mut commands: Commands,
    assets: Res<GameAssets>,
    level: Res<Level>,
) -> SceneResult {
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
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(UiTextBundle {
                        style: Style {
                            margin: Rect {
                                top: Val::Px(15.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: LEVEL_NAMES[level.0].to_string(),
                                style: TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 30.0,
                                    color: Color::YELLOW,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    parent.spawn().insert_bundle(UiTextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Level {}", level.0 + 1),
                                style: TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 30.0,
                                    color: Color::RED,
                                },
                            }],
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
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(UiTextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "STISKNI ENTER".to_string(),
                                style: TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 25.0,
                                    color: Color::ORANGE_RED,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .id();

    commands.insert_resource(LevelOpeningData { root });

    SceneResult::Ok(SceneStage::Update)
}

fn level_opening_update(
    mut commands: Commands,
    keyboard: Res<KeyboardInput>,
    data: Res<LevelOpeningData>,
) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Return) {
        commands.entity(data.root).despawn_recursive();
        commands.remove_resource::<LevelOpeningData>();
        SceneResult::Replace(Box::new(LevelScene::default()), SceneStage::Start)
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

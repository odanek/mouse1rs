use quad::prelude::*;

use crate::{
    level::{Level, LevelScene},
    mouse::{GameAssets, Lifes, render_lifes},
};

pub struct LostLifeSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

#[derive(Resource)]
struct LostLifeData {
    root: Entity,
}

#[derive(Default)]
pub struct LostLifeScene {
    schedule: Option<LostLifeSchedule>,
}

impl Scene for LostLifeScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| LostLifeSchedule {
            start: Scheduler::chain(world)
                .add(render_lifes)
                .add(lost_life_start)
                .build(),
            update: Scheduler::single(lost_life_update),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn lost_life_start(
    mut commands: Commands,
    assets: Res<GameAssets>,
    lifes: Res<Lifes>,
) -> SceneResult {
    let message = if lifes.count == 0 {
        "Zemrel jsi"
    } else {
        "Ztratil jsi zivot"
    };

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
                        text: Text {
                            sections: vec![TextSection {
                                value: message.to_string(),
                                style: TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 30.0,
                                    color: Color::GREEN,
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

    commands.insert_resource(LostLifeData { root });

    SceneResult::Ok(SceneStage::Update)
}

fn lost_life_update(
    mut commands: Commands,
    keyboard: Res<KeyboardInput>,
    data: Res<LostLifeData>,
    lifes: Res<Lifes>,
) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Enter) {
        commands.entity(data.root).despawn_recursive();
        commands.remove_resource::<LostLifeData>();
        if lifes.count == 0 {
            commands.remove_resource::<Level>();
            SceneResult::Pop(SceneStage::Resume)
        } else {
            SceneResult::Replace(Box::<LevelScene>::default(), SceneStage::Start)
        }
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

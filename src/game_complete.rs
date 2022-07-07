use quad::prelude::*;

use crate::{mouse::render_lifes, mouse::GameAssets};

pub struct GameCompleteSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

#[derive(Resource)]
struct GameCompleteData {
    root: Entity,
}

#[derive(Default)]
pub struct GameCompleteScene {
    schedule: Option<GameCompleteSchedule>,
}

impl Scene for GameCompleteScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| GameCompleteSchedule {
            start: Scheduler::chain(world)
                .add(&render_lifes)
                .add(&game_complete_start)
                .build(),
            update: Scheduler::single(game_complete_update.system(world)),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn game_complete_start(mut commands: Commands, assets: Res<GameAssets>) -> SceneResult {
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
                                value: "Blahopreji dokoncil jsi hru!".to_string(),
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

    commands.insert_resource(GameCompleteData { root });

    SceneResult::Ok(SceneStage::Update)
}

fn game_complete_update(
    mut commands: Commands,
    keyboard: Res<KeyboardInput>,
    data: Res<GameCompleteData>,
) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Return) {
        commands.entity(data.root).despawn_recursive();
        commands.remove_resource::<GameCompleteData>();
        SceneResult::Pop(SceneStage::Resume)
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

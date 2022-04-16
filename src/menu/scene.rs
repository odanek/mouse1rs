use quad::{
    ecs::{Commands, IntoSystem, Res, ResMut, Schedule, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    render::color::Color,
    text::{
        HorizontalAlign, Text, TextAlignment, TextBundle, TextSection, TextStyle, VerticalAlign,
    },
    transform::Transform,
    windowing::Windows,
    Scene, SceneResult,
};

use crate::mouse::GameAssets;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MenuStage {
    Start,
    Update,
}

struct MenuSceneSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

pub struct MenuScene {
    stage: MenuStage,
    schedule: Option<MenuSceneSchedule>,
}

impl Scene for MenuScene {
    fn update(&mut self, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| MenuSceneSchedule {
            start: Scheduler::single(menu_init.system(world)),
            update: Scheduler::single(menu_update.system(world)),
        });

        match self.stage {
            MenuStage::Start => {
                self.stage = MenuStage::Update;
                schedule.start.run(world)
            }
            MenuStage::Update => schedule.update.run(world),
        }
    }
}

impl Default for MenuScene {
    fn default() -> Self {
        Self {
            stage: MenuStage::Start,
            schedule: None,
        }
    }
}

fn menu_init(
    mut commands: Commands,
    assets: Res<GameAssets>,
    windows: ResMut<Windows>,
) -> SceneResult {
    let window_size = windows.primary().size();

    commands.spawn().insert_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "The ".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 30.0,
                        color: Color::BLUE,
                    },
                },
                TextSection {
                    value: " Mouse".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 30.0,
                        color: Color::GREEN,
                    },
                },
            ],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Top,
            },
        },
        transform: Transform::from_xyz(0.0, window_size.height / 2.0, 0.0),
        ..Default::default()
    });

    commands.spawn().insert_bundle(TextBundle {
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
    });

    commands.spawn().insert_bundle(TextBundle {
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
    });

    commands.spawn().insert_bundle(TextBundle {
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
    });

    SceneResult::Ok
}

fn menu_update(keyboard: Res<KeyboardInput>) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Key2) {
        SceneResult::Pop
    } else {
        SceneResult::Ok
    }
}

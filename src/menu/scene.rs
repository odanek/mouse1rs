use quad::{
    ecs::{Commands, IntoSystem, Res, Schedule, Scheduler, World, ResMut},
    input::{KeyCode, KeyboardInput},
    render::color::Color,
    text::{Text, TextBundle, TextSection, TextStyle, TextAlignment, HorizontalAlign, VerticalAlign},
    Scene, SceneResult, windowing::Windows, transform::Transform,
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

fn menu_init(mut commands: Commands, assets: Res<GameAssets>, windows: ResMut<Windows>) -> SceneResult {
    let window_size = windows.primary().size();
    println!("{:?}", window_size);

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
                vertical: VerticalAlign::Top
            }
        },
        transform: Transform::from_xyz(0.0, window_size.height / 2.0, 0.0),
        ..Default::default()
    });
    SceneResult::Ok
}

fn menu_update(keyboard: Res<KeyboardInput>) -> SceneResult {
    if keyboard.just_pressed(KeyCode::Escape) {
        SceneResult::Pop
    } else {
        SceneResult::Ok
    }
}

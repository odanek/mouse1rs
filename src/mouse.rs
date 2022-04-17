use quad::{
    asset::{AssetServer, Handle},
    ecs::{Commands, IntoSystem, Res, ResMut, Resource, Scheduler, World},
    pipeline::ClearColor,
    render::color::Color,
    text::{
        Font, HorizontalAlign, Text, TextAlignment, TextBundle, TextSection, TextStyle,
        VerticalAlign,
    },
    transform::Transform,
    windowing::Windows,
    Scene, SceneResult, SceneStage,
};

use crate::menu::MenuScene;

#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
}

#[derive(Default)]
pub struct MouseScene {}

impl Scene for MouseScene {
    fn update(&mut self, _stage: SceneStage, world: &mut World) -> SceneResult {
        Scheduler::single(bootstrap.system(world)).run(world)
    }
}

fn bootstrap(
    mut commands: Commands,
    assets: Res<AssetServer>,
    windows: ResMut<Windows>,
) -> SceneResult {
    let font = assets.load("helvetica.ttf");

    commands.insert_resource(ClearColor(Color::BLACK));
    commands.insert_resource(GameAssets { font: font.clone() });

    let window_size = windows.primary().size();

    commands.spawn().insert_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "The ".to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::BLUE,
                    },
                },
                TextSection {
                    value: " Mouse".to_string(),
                    style: TextStyle {
                        font,
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

    SceneResult::Replace(Box::new(MenuScene::default()))
}

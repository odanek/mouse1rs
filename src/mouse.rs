use quad::{
    asset::{AssetServer, Handle},
    ecs::{Commands, IntoSystem, Res, Resource, Scheduler, World},
    pipeline::ClearColor,
    render::color::Color,
    text::{Font, Text, TextSection, TextStyle},
    ty::Size,
    ui::{
        entity::{NodeBundle, UiTextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, Val,
    },
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

fn bootstrap(mut commands: Commands, assets: Res<AssetServer>) -> SceneResult {
    let font = assets.load("helvetica.ttf");

    commands.insert_resource(ClearColor(Color::BLACK));
    commands.insert_resource(GameAssets { font: font.clone() });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(UiTextBundle {
                style: Default::default(),
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
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    SceneResult::Replace(Box::new(MenuScene::default()), SceneStage::Start)
}

use quad::{
    asset::{AssetServer, Handle, Assets},
    ecs::{Commands, IntoSystem, Res, Resource, Scheduler, World, Schedule, ResMut},
    pipeline::ClearColor,
    render::{AddressMode, color::Color, texture::Image},
    text::{Font, Text, TextSection, TextStyle},
    ty::Size,
    ui::{
        entity::{NodeBundle, UiTextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, Val,
    },
    Scene, SceneResult, SceneStage,
};

use crate::{menu::MenuScene, level::Level};

#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub background: Handle<Image>,
    pub foreground: Handle<Image>,
}

pub struct MouseSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

#[derive(Default)]
pub struct MouseScene {
    schedule: Option<MouseSchedule>,
}

impl Scene for MouseScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| MouseSchedule {
            start: Scheduler::single(mouse_start.system(world)),
            update: Scheduler::single(mouse_update.system(world)),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn mouse_start(mut commands: Commands, asset_server: Res<AssetServer>) -> SceneResult {
    commands.insert_resource(ClearColor(Color::BLACK));

    let foreground = asset_server.load(Level(0).fg_path());
    let background = asset_server.load(Level(0).bg_path());
    let font = asset_server.load("helvetica.ttf");

    commands.insert_resource(GameAssets { 
        font: font.clone(),
        background,
        foreground,
    });

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

    SceneResult::Ok(SceneStage::Update)
}

fn mouse_update(
    game_assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
) -> SceneResult {
    if images.get(&game_assets.foreground).is_some() {
        if let Some(bg_image) = images.get_mut(&game_assets.background) {
            bg_image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
            return SceneResult::Replace(Box::new(MenuScene::default()), SceneStage::Start)
        }
    }

    SceneResult::Ok(SceneStage::Update)
}

use std::path::PathBuf;

use quad::{
    asset::{AssetServer, Assets, Handle},
    ecs::{Commands, IntoSystem, Res, ResMut, Resource, Schedule, Scheduler, World},
    pipeline::ClearColor,
    render::{color::Color, texture::Image, AddressMode},
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
    pub background: Vec<Handle<Image>>,
    pub foreground: Vec<Handle<Image>>,
}

impl GameAssets {
    pub fn foreground_path(level: u32) -> PathBuf {
        format!("levels/level{}.tga", level).into()
    }

    pub fn background_path(level: u32) -> PathBuf {
        format!("levels/level{}.bcg.tga", level).into()
    }
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

    let foreground = (0..1)
        .map(|level| asset_server.load(GameAssets::foreground_path(level)))
        .collect();
    let background = (0..1)
        .map(|level| asset_server.load(GameAssets::background_path(level)))
        .collect();
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

fn mouse_update(game_assets: Res<GameAssets>, mut images: ResMut<Assets<Image>>) -> SceneResult {
    let all_fg_loaded = game_assets
        .foreground
        .iter()
        .all(|handle| images.contains(handle));
    let all_bcg_loaded = game_assets
        .background
        .iter()
        .all(|handle| images.contains(handle));

    if all_fg_loaded && all_bcg_loaded {
        for handle in game_assets.background.iter() {
            let mut image = images.get_mut(handle).unwrap();
            image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
        }
        SceneResult::Replace(Box::new(MenuScene::default()), SceneStage::Start)
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

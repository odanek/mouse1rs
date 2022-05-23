use quad::{
    asset::{AssetServer, Assets, Handle},
    ecs::{Commands, IntoSystem, Res, ResMut, Resource, Schedule, Scheduler, World},
    pipeline::ClearColor,
    render::{color::Color, texture::Image, AddressMode},
    run::{Scene, SceneResult, SceneStage},
    text::{Font, Text, TextSection, TextStyle},
    ty::Size,
    ui::{
        entity::{NodeBundle, UiTextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, Val,
    },
};

use crate::{hit_map::HitMap, level::LevelAssets, menu::MenuScene};

#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub level: Vec<LevelAssets>,
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

    let font = asset_server.load("helvetica.ttf");
    let level = (0..5)
        .map(|level| LevelAssets {
            foreground: asset_server.load(LevelAssets::foreground_path(level)),
            background: asset_server.load(LevelAssets::background_path(level)),
            hit_map: asset_server.load(LevelAssets::hit_map_path(level)),
        })
        .collect();

    commands.insert_resource(GameAssets {
        font: font.clone(),
        level,
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
    hit_maps: Res<Assets<HitMap>>,
) -> SceneResult {
    let levels_loaded = game_assets.level.iter().all(|level| {
        images.contains(&level.foreground)
            && images.contains(&level.background)
            && hit_maps.contains(&level.hit_map)
    });

    if levels_loaded {
        for level in game_assets.level.iter() {
            let mut image = images.get_mut(&level.background).unwrap();
            image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
        }
        SceneResult::Replace(Box::new(MenuScene::default()), SceneStage::Start)
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

use quad::{
    ecs::{Commands, Component, Entity, Query, Res, ResMut, Resource, Schedule, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    render::cameras::Camera2d,
    run::{Scene, SceneResult, SceneStage},
    sprite::{Rect, Sprite, SpriteBundle},
    timing::Time,
    transform::{Transform, TransformBundle},
    ty::{Vec2, Vec3},
    windowing::Windows,
};

use crate::mouse::GameAssets;

const TITLE_HEIGHT: f32 = 30.0;
const SCREEN_WIDTH: f32 = 320.0;
const SCREEN_HEIGHT: f32 = 192.0;
const SCREEN_COUNT: f32 = 10.0;
const BCG_SCREEN_COUNT: f32 = SCREEN_COUNT / 2.0 + 0.5;

#[derive(Resource)]
pub struct Level(pub usize);

#[derive(Component)]
pub struct BackgroundImage;

#[derive(Resource)]
struct LevelData {
    camera_position: f32,
    root: Entity,
    zoom: f32,
    quit: bool,
}

pub struct LevelSchedule {
    start: Schedule<(), SceneResult>,
    update: Schedule<(), SceneResult>,
}

#[derive(Default)]
pub struct LevelScene {
    schedule: Option<LevelSchedule>,
}

impl Scene for LevelScene {
    fn update(&mut self, stage: SceneStage, world: &mut World) -> SceneResult {
        let schedule = self.schedule.get_or_insert_with(|| LevelSchedule {
            start: Scheduler::chain(world)
                .add(level_start)
                // .add(position_camera)  // TODO How to fix?
                // .add(position_background)
                .add(finalize_start)
                .build(),
            update: Scheduler::chain(world)
                .add(&handle_input)
                .add(&position_camera)
                .add(&position_background)
                .add(&finalize_update)
                .build(),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn level_start(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    level: Res<Level>,
    windows: ResMut<Windows>,
    mut camera: Query<(&Camera2d, &mut Transform)>,
) {
    let window_size = windows.primary().size();
    let zoom = (window_size.height - TITLE_HEIGHT) / SCREEN_HEIGHT;
    let camera_position = SCREEN_WIDTH * SCREEN_COUNT / 2.0 - SCREEN_WIDTH / 2.0;

    let background = commands
        .spawn()
        .insert(BackgroundImage)
        .insert_bundle(SpriteBundle {
            texture: game_assets.background[level.0].clone(),
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2::new(0.0, 0.0),
                    max: Vec2::new(BCG_SCREEN_COUNT * SCREEN_WIDTH, SCREEN_HEIGHT),
                }),
                custom_size: Some(Vec2::new(BCG_SCREEN_COUNT * SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xy(camera_position / 2.0, 0.0),
            ..Default::default()
        })
        .id();

    let foreground = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: game_assets.foreground[level.0].clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .id();

    let root = commands
        .spawn()
        .push_children(&[foreground, background])
        .insert_bundle(TransformBundle {
            local: Transform {
                scale: Vec3::new(zoom, zoom, 1.0),
                translation: Vec3::new(0.0, -TITLE_HEIGHT / 2.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.insert_resource(LevelData {
        camera_position,
        root,
        zoom,
        quit: false,
    });

    if let Ok((_, mut camera_pos)) = camera.single_mut() {
        camera_pos.translation.x = camera_position * zoom;
    }
}

fn finalize_start() -> SceneResult {
    SceneResult::Ok(SceneStage::Update)
}

fn handle_input(time: Res<Time>, mut level_data: ResMut<LevelData>, keyboard: Res<KeyboardInput>) {
    if keyboard.pressed(KeyCode::Left) {
        level_data.camera_position -= 200.0 * time.delta_seconds();
    } else if keyboard.pressed(KeyCode::Right) {
        level_data.camera_position += 200.0 * time.delta_seconds();
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        level_data.quit = true;
    }
}

fn position_camera(level_data: Res<LevelData>, mut camera: Query<(&Camera2d, &mut Transform)>) {
    if let Ok((_, mut camera_pos)) = camera.single_mut() {
        camera_pos.translation.x = level_data.camera_position * level_data.zoom;
    }
}

fn position_background(
    level_data: Res<LevelData>,
    mut background: Query<(&BackgroundImage, &mut Transform)>,
) {
    if let Ok((_, mut background_pos)) = background.single_mut() {
        background_pos.translation.x = level_data.camera_position / 2.0;
    }
}

fn finalize_update(mut commands: Commands, level_data: ResMut<LevelData>) -> SceneResult {
    if level_data.quit {
        commands.entity(level_data.root).despawn_recursive();
        commands.remove_resource::<Level>();
        commands.remove_resource::<LevelData>();
        SceneResult::Pop(SceneStage::Resume)
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

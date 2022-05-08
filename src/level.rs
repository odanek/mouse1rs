use std::path::PathBuf;

use quad::{
    asset::AssetServer,
    ecs::{Commands, Entity, IntoSystem, Query, Res, ResMut, Resource, Schedule, Scheduler, World},
    input::{KeyCode, KeyboardInput},
    render::{cameras::Camera2d},
    sprite::SpriteBundle,
    transform::{Transform, TransformBundle},
    ty::Vec3,
    windowing::Windows,
    Scene, SceneResult, SceneStage, timing::Time,
};

#[derive(Resource)]
pub struct Level(pub u32);

impl Level {
    pub fn fg_path(&self) -> PathBuf {
        format!("levels/level{}.tga", self.0 + 1).into()
    }

    pub fn bg_path(&self) -> PathBuf {
        format!("levels/level{}.bcg.tga", self.0 + 1).into()
    }
}

#[derive(Resource)]
struct LevelData {
    root: Entity,
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
            start: Scheduler::single(level_init.system(world)),
            update: Scheduler::single(level_update.system(world)),
        });

        match stage {
            SceneStage::Start => schedule.start.run(world),
            SceneStage::Update => schedule.update.run(world),
            _ => unreachable!(),
        }
    }
}

fn level_init(
    mut commands: Commands,
    level: Res<Level>,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
    mut camera: Query<(&Camera2d, &mut Transform)>,
) -> SceneResult {
    let window_size = windows.primary().size();
    let zoom = (window_size.height - 30.0) / 192.0;
    let initial_x =0.0; //4.5 * 320.0;

    let fg_texture = asset_server.load(level.fg_path());
    let bg_texture = asset_server.load(level.bg_path());

    let bg = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: bg_texture,
            transform: Transform::from_xy(initial_x, 0.0),
            ..Default::default()
        })
        .id();

    let fg = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: fg_texture,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .id();

    let root = commands
        .spawn()
        .push_children(&[fg, bg])
        .insert_bundle(TransformBundle {
            local: Transform {
                scale: Vec3::new(zoom, zoom, 1.0),
                translation: Vec3::new(0.0, -15.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.insert_resource(LevelData { root });

    if let Ok(mut camera2d) = camera.single_mut() {
        camera2d.1.translation.x = initial_x * zoom;
    }

    SceneResult::Ok(SceneStage::Update)
}

fn level_update(
    mut commands: Commands,
    time: Res<Time>,
    level_data: Res<LevelData>,
    keyboard: Res<KeyboardInput>,
    mut camera: Query<(&Camera2d, &mut Transform)>,
) -> SceneResult {
    if let Ok((_, mut camera_pos)) = camera.single_mut() {
        if keyboard.pressed(KeyCode::Left) {
            camera_pos.translation.x -= 500.0 * time.delta_seconds();
        } else if keyboard.pressed(KeyCode::Right) {
            camera_pos.translation.x += 500.0 * time.delta_seconds();
        }    
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        commands.entity(level_data.root).despawn_recursive();
        commands.remove_resource::<LevelData>();
        SceneResult::Pop(SceneStage::Resume)
    } else {
        SceneResult::Ok(SceneStage::Update)
    }
}

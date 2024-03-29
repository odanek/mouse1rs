use std::path::PathBuf;

use quad::prelude::*;

use crate::{
    constant::*,
    game_complete::GameCompleteScene,
    hit_map::HitMap,
    level_opening::LevelOpeningScene,
    lost_life::LostLifeScene,
    mouse::{GameAssets, Lifes},
    player::{Player, PlayerOrientation, PlayerState},
};

#[derive(Resource)]
pub struct Level(pub usize);

pub struct LevelAssets {
    pub background: Handle<Image>,
    pub foreground: Handle<Image>,
    pub hit_map: Handle<HitMap>,
}

impl LevelAssets {
    pub fn foreground_path(level: u32) -> PathBuf {
        format!("levels/{level}/fg.tga").into()
    }

    pub fn background_path(level: u32) -> PathBuf {
        format!("levels/{level}/bcg.tga").into()
    }

    pub fn hit_map_path(level: u32) -> PathBuf {
        format!("levels/{level}/map.hit").into()
    }
}

#[derive(Component)]
pub struct BackgroundImage;

#[derive(Component)]
pub struct SceneRoot;

#[derive(Copy, Clone, PartialEq, Eq)]
enum LevelState {
    Play,
    Quit,
    Dead,
    Next,
}

#[derive(Resource)]
struct LevelData {
    state: LevelState,
    camera_position: f32,
    camera_max: f32,
    camera_min: f32,
    root: Entity,
    zoom: f32,
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
                .add(finalize_start)
                .build(),
            update: Scheduler::chain(world)
                .add(update_player)
                .add(handle_input)
                .add(update_zoom)
                .add(position_camera)
                .add(position_background)
                .add(finalize_update)
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
    windows: Res<Windows>,
    hit_map_assets: Res<Assets<HitMap>>,
    mut camera: Query<(&Camera2d, &mut Transform)>,
) {
    let window_size = windows.primary().size();
    let hit_map = hit_map_assets
        .get(&game_assets.level[level.0].hit_map)
        .unwrap();

    let player_position = Vec2::new(3180.0, 50.0);
    let player = Player {
        orientation: PlayerOrientation::Left,
        state: if hit_map.check_bottom(player_position.x, player_position.y + 1.0) {
            PlayerState::Standing
        } else {
            PlayerState::Falling
        },
        position: player_position,
        jump_phase: 0.0,
        animation_phase: 0.0,
    };

    let (zoom, camera_min, camera_max) = camera_properties(window_size);
    let camera_position = camera_max;

    let background = commands
        .spawn()
        .insert(BackgroundImage)
        .insert_bundle(SpriteBundle {
            texture: game_assets.level[level.0].background.clone(),
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2::new(0.0, 0.0),
                    max: Vec2::new(BCG_SCREEN_COUNT * SCREEN_WIDTH, SCREEN_HEIGHT),
                }),
                ..Default::default()
            },
            transform: Transform::from_xy(camera_position / 2.0, 0.0),
            ..Default::default()
        })
        .id();

    let foreground = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: game_assets.level[level.0].foreground.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .id();

    let player = commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.player.clone(),
            sprite: TextureAtlasSprite {
                index: player.sprite_index(),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                player.position.x + PLAYER_X_OFFSET,
                PLAYER_Y_OFFSET - player.position.y,
                2.0,
            ),
            ..Default::default()
        })
        .insert(player)
        .id();

    let root = commands
        .spawn()
        .push_children(&[foreground, background, player])
        .insert(SceneRoot)
        .insert_bundle(SpatialBundle {
            transform: Transform {
                scale: Vec3::new(zoom, zoom, 1.0),
                translation: Vec3::new(0.0, -TITLE_HEIGHT / 2.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.insert_resource(LevelData {
        state: LevelState::Play,
        camera_position,
        camera_min,
        camera_max,
        root,
        zoom,
    });

    if let Ok((_, mut camera_pos)) = camera.get_single_mut() {
        camera_pos.translation.x = camera_position * zoom;
    }
}

fn finalize_start() -> SceneResult {
    SceneResult::Ok(SceneStage::Update)
}

fn update_player(
    time: Res<Time>,
    keyboard: Res<KeyboardInput>,
    game_assets: Res<GameAssets>,
    level: Res<Level>,
    mut level_data: ResMut<LevelData>,
    hit_map_assets: Res<Assets<HitMap>>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut TextureAtlasSprite)>,
) {
    let (mut player, mut transform, mut sprite) = player_query.single_mut();
    let hit_map = hit_map_assets
        .get(&game_assets.level[level.0].hit_map)
        .unwrap();

    if player.state == PlayerState::Standing && player.can_fall(hit_map) {
        player.state = PlayerState::Falling;
    }

    if keyboard.pressed(KeyCode::ArrowUp) {
        player.jump(hit_map);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        player.move_left(time.as_ref(), hit_map)
    } else if keyboard.pressed(KeyCode::ArrowRight) {
        player.move_right(time.as_ref(), hit_map)
    }

    if player.state == PlayerState::Jumping {
        player.move_up(time.as_ref(), hit_map);
    }
    if player.state == PlayerState::Falling {
        player.move_down(time.as_ref(), hit_map);
    }
    if player.state == PlayerState::Standing {
        if player.is_dead(hit_map) {
            level_data.state = LevelState::Dead;
        } else if player.is_next_level(hit_map) {
            level_data.state = LevelState::Next;
        }
    }

    transform.translation.x = player.position.x + PLAYER_X_OFFSET;
    transform.translation.y = PLAYER_Y_OFFSET - player.position.y;
    sprite.index = player.sprite_index();
}

fn handle_input(mut level_data: ResMut<LevelData>, keyboard: Res<KeyboardInput>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        level_data.state = LevelState::Quit;
    }
    if keyboard.pressed(KeyCode::KeyM)
        && keyboard.pressed(KeyCode::KeyY)
        && keyboard.pressed(KeyCode::KeyS)
    {
        level_data.state = LevelState::Next;
    }
}

fn update_zoom(
    windows: Res<Windows>,
    mut level_data: ResMut<LevelData>,
    mut root: Query<(&SceneRoot, &mut Transform)>,
) {
    let window_size = windows.primary().size();
    let (zoom, camera_min, camera_max) = camera_properties(window_size);

    level_data.camera_min = camera_min;
    level_data.camera_max = camera_max;
    level_data.camera_position = level_data.camera_position.clamp(camera_min, camera_max);
    level_data.zoom = zoom;

    if let Ok((_, mut root_pos)) = root.get_single_mut() {
        root_pos.scale.x = zoom;
        root_pos.scale.y = zoom;
    }
}

fn position_camera(
    mut level_data: ResMut<LevelData>,
    player_query: Query<&Player>,
    mut camera_query: Query<(&Camera2d, &mut Transform)>,
) {
    let player = player_query.single();
    let player_x = player.position.x + PLAYER_X_OFFSET;
    level_data.camera_position = level_data
        .camera_position
        .clamp(player_x - 40.0, player_x + 40.0)
        .clamp(level_data.camera_min, level_data.camera_max);

    let (_, mut camera_pos) = camera_query.single_mut();
    camera_pos.translation.x = level_data.camera_position * level_data.zoom;
}

fn position_background(
    level_data: Res<LevelData>,
    mut background: Query<(&BackgroundImage, &mut Transform)>,
) {
    if let Ok((_, mut background_pos)) = background.get_single_mut() {
        background_pos.translation.x = level_data.camera_position / 2.0;
    }
}

fn finalize_update(
    mut commands: Commands,
    level_data: ResMut<LevelData>,
    mut level: ResMut<Level>,
    mut lifes: ResMut<Lifes>,
) -> SceneResult {
    match level_data.state {
        LevelState::Quit => {
            commands.entity(level_data.root).despawn_recursive();
            commands.remove_resource::<Level>();
            commands.remove_resource::<LevelData>();
            lifes.count = 0;
            SceneResult::Pop(SceneStage::Resume)
        }
        LevelState::Dead => {
            lifes.count -= 1;
            commands.entity(level_data.root).despawn_recursive();
            commands.remove_resource::<LevelData>();
            SceneResult::Replace(Box::<LostLifeScene>::default(), SceneStage::Start)
        }
        LevelState::Next => {
            commands.entity(level_data.root).despawn_recursive();
            commands.remove_resource::<LevelData>();
            if level.0 == 4 {
                commands.remove_resource::<Level>();
                lifes.count = 0;
                SceneResult::Replace(Box::<GameCompleteScene>::default(), SceneStage::Start)
            } else {
                level.0 += 1;
                SceneResult::Replace(Box::<LevelOpeningScene>::default(), SceneStage::Start)
            }
        }
        _ => SceneResult::Ok(SceneStage::Update),
    }
}

fn camera_properties(window_size: Vec2) -> (f32, f32, f32) {
    let viewport_height = window_size.y - TITLE_HEIGHT;
    let zoom = viewport_height / SCREEN_HEIGHT;
    let aspect = window_size.x / viewport_height;
    let camera_max = (TOTAL_SCREEN_WIDTH - SCREEN_HEIGHT * aspect) / 2.0;
    let camera_min = (-TOTAL_SCREEN_WIDTH + SCREEN_HEIGHT * aspect) / 2.0;
    (zoom, camera_min, camera_max)
}

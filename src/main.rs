mod constant;
mod game_complete;
mod hit_map;
mod level;
mod level_opening;
mod lost_life;
mod menu;
mod mouse;
mod player;

use hit_map::{HitMap, HitMapLoader};
use mouse::MouseScene;
use quad::prelude::*;

fn main() {
    Quad::new(QuadConfig {
        main_window: WindowDescriptor {
            title: "The Mouse 1".to_string(),
            size: LogicalSize {
                width: 960.0,
                height: 600.0,
            }
            .into(),
        },
        ..Default::default()
    })
    .add_asset::<HitMap>()
    .init_asset_loader::<HitMapLoader>()
    .run(Box::<MouseScene>::default());
}

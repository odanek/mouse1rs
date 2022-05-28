use std::iter::repeat;

use quad::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    ty::{BoxedFuture, Vec2},
};

pub struct HitMap {
    map: Vec<u8>,
}

impl HitMap {
    pub fn from_rle_bytes(bytes: &[u8]) -> Self {
        let mut map = Vec::with_capacity(320 * 192 * 10);
        let mut count = 0;
        for rle in bytes.chunks(2) {
            map.extend(repeat(rle[1]).take(rle[0] as usize));
            count += rle[0] as usize;
        }
        println!("Loaded {}", count);
        Self { map }
    }

    pub fn check_collision(position: Vec2) -> bool {
        false
    }
}

#[derive(Default)]
pub struct HitMapLoader;

impl AssetLoader for HitMapLoader {
    fn load(
        &self,
        bytes: &[u8],
        load_context: &mut LoadContext,
    ) -> BoxedFuture<anyhow::Result<()>> {
        let hit_map = HitMap::from_rle_bytes(bytes);
        load_context.set_default_asset(LoadedAsset::new(hit_map));
        Box::pin(async move { Ok(()) })
    }

    fn extensions(&self) -> &[&str] {
        &["hit"]
    }
}

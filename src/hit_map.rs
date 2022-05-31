use std::iter::repeat;

use quad::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    ty::{BoxedFuture},
};

const ROW_PIXEL_COUNT: usize = 320 * 10;

pub struct HitMap {
    map: Vec<u8>,
}

impl HitMap {
    pub fn from_rle_bytes(bytes: &[u8]) -> Self {
        let mut map = Vec::with_capacity(320 * 192 * 10);
        for rle in bytes.chunks(2) {
            map.extend(repeat(rle[1]).take(rle[0] as usize));
        }
        Self { map }
    }

    pub fn check_collision(&self, x: f32, y: f32) -> bool {
        if x + 0.5 < 0.0 || y + 0.5 < 0.0 {            
            return true;
        }

        let px = (x + 0.5) as usize;
        let py = (y + 0.5) as usize;
        let mut index = (py * ROW_PIXEL_COUNT) + px;

        for _ in 0..16 {
            for _ in 0..10 {
                if self.map[index] == 1 {
                    return true;
                }
                index += 1;
            }
            index += ROW_PIXEL_COUNT - 10;
        }
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

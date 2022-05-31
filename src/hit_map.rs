use std::iter::repeat;

use quad::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    ty::BoxedFuture,
};

use crate::constant::{SCREEN_HEIGHT, TOTAL_SCREEN_WIDTH};

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

    pub fn kind_at(&self, x: f32, y: f32) -> Option<u8> {
        let ax = x + 0.5;
        let ay = y + 0.5;

        if ax < 0.0 || ay < 0.0 || ax >= TOTAL_SCREEN_WIDTH || ay >= SCREEN_HEIGHT {
            return None;
        }

        let px = ax as usize;
        let py = ay as usize;
        Some(self.map[py * ROW_PIXEL_COUNT + px])
    }

    pub fn is_block(&self, x: f32, y: f32) -> bool {
        return matches!(self.kind_at(x, y), Some(1) | None);
    }

    pub fn check_left(&self, x: f32, y: f32) -> bool {
        for yo in 0..16 {
            if self.is_block(x, y + yo as f32) {
                return true;
            }
        }
        false
    }

    pub fn check_right(&self, x: f32, y: f32) -> bool {
        for yo in 0..16 {
            if self.is_block(x + 9.0, y + yo as f32) {
                return true;
            }
        }
        false
    }

    pub fn check_top(&self, x: f32, y: f32) -> bool {
        for xo in 0..10 {
            if self.is_block(x + xo as f32, y) {
                return true;
            }
        }
        false
    }

    pub fn check_bottom(&self, x: f32, y: f32) -> bool {
        for xo in 0..10 {
            if self.is_block(x + xo as f32, y + 15.0) {
                return true;
            }
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

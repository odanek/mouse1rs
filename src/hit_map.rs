use quad::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    ty::{BoxedFuture, Vec2},
};

pub struct HitMap {
    map: Vec<u8>,
}

impl HitMap {
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
        let hit_map = HitMap {
            map: bytes.to_vec(),
        };
        load_context.set_default_asset(LoadedAsset::new(hit_map));
        Box::pin(async move { Ok(()) })
    }

    fn extensions(&self) -> &[&str] {
        &["hit"]
    }
}

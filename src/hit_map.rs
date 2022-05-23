use quad::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    ty::BoxedFuture,
};
use uuid::{uuid, Uuid};

pub struct HitMap {
    map: Vec<u8>,
}

impl TypeUuid for HitMap {
    const TYPE_UUID: Uuid = uuid!("add1f429-aef9-4134-9bb6-7cfebef86ecb");
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

use quad::{asset::Handle, ecs::Resource, text::Font};

#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
}

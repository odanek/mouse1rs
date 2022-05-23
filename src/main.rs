mod level;
mod menu;
mod mouse;

use mouse::MouseScene;
use quad::{
    run::{Quad, QuadConfig},
    windowing::{LogicalSize, WindowDescriptor},
};

fn main() {
    // TODO Prepocitat zoom at odpovida velikost pixelu CRT monitorum

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
    .run(Box::new(MouseScene::default()));
}

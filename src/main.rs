mod logging;
mod menu;
mod mouse;

use logging::init_logging;
use mouse::MouseScene;
use quad::{
    windowing::{LogicalSize, WindowDescriptor},
    Quad, QuadConfig,
};

fn main() {
    init_logging();

    Quad::new(QuadConfig {
        main_window: WindowDescriptor {
            title: "The Mouse 1".to_string(),
            size: LogicalSize {
                width: 800.0,
                height: 600.0,
            }
            .into(),
        },
        ..Default::default()
    })
    .run(Box::new(MouseScene::default()));
}

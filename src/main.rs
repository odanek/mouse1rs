mod level;
mod logging;
mod menu;
mod mouse;
mod preload;

use logging::init_logging;
use mouse::MouseScene;
use quad::{
    windowing::{LogicalSize, WindowDescriptor},
    Quad, QuadConfig,
};

fn main() {
    init_logging(); // TODO Move to quad

    // TODO Nahrat vsechny textury a data dopredu pred zobrazenim menu
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

mod logging;
mod menu;

use logging::init_logging;
use menu::MenuScene;
use quad::{
    windowing::{LogicalSize, Window},
    Quad, QuadConfig,
};

fn main() {
    init_logging();

    let config = QuadConfig {
        main_window: Window::builder()
            .title("The Mouse 1")
            .inner_size(LogicalSize {
                width: 800.0,
                height: 600.0,
            }),
        ..Default::default()
    };

    Quad::new(&config).run(Box::new(MenuScene::default()));
}

use crate::data::AppData;
use crate::list_connection::list_connection;
use druid::{Point, WindowConfig, WindowDesc, WindowSizePolicy};

pub fn init_window() -> WindowDesc<AppData> {
    let mut config = WindowConfig::default()
        .resizable(true)
        .window_size((300.0, 1500.0))
        .set_position(Point::new(300f64, 0f64))
        .show_titlebar(false)
        // .set_level()
        // .transparent()
        .window_size_policy(WindowSizePolicy::User);
    WindowDesc::new(list_connection()).with_config(config)
}

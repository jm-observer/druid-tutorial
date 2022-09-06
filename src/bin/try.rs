use druid::{AppLauncher, PlatformError, WindowDesc};
use druid_tutorial::data::hierarchy::AppData;
use druid_tutorial::ui::layout::init_layout;
use druid_tutorial::ui::menu::menu;

fn main() -> Result<(), PlatformError> {
    custom_utils::logger::logger_stdout_debug();
    let win = WindowDesc::new(init_layout()).menu(menu);
    AppLauncher::with_window(win).launch(AppData::default())?;
    Ok(())
}

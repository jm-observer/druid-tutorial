use druid::{AppLauncher, PlatformError, WindowDesc};
use druid_tutorial::data::AppData;
use druid_tutorial::ui::layout::init_layout;
use druid_tutorial::ui::menu::menu;

fn main() -> Result<(), PlatformError> {
    let win = WindowDesc::new(init_layout()).menu(menu);
    AppLauncher::with_window(win).launch(AppData::default())?;
    Ok(())
}

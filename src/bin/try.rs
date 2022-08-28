use druid::widget::{Align, Flex, Label, Padding};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};
use druid_tutorial::data::AppData;
use druid_tutorial::ui::brokers::list::init_connect;
use druid_tutorial::ui::layout::init_layout;
use druid_tutorial::ui::menu::menu;

fn main() -> Result<(), PlatformError> {
    let win = WindowDesc::new(init_layout()).menu(menu);
    AppLauncher::with_window(win)
        // .with_window(win)
        .launch(AppData::default())?;
    Ok(())
}

use druid::widget::{Align, Flex, Label, Padding};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};
use druid_tutorial::data::AppData;
use druid_tutorial::ui::brokers::list::init_connect;

fn main() -> Result<(), PlatformError> {
    let win = WindowDesc::new(init_connect());
    AppLauncher::new()
        .with_window(win)
        // .with_window(win)
        .launch(AppData::default())?;
    Ok(())
}

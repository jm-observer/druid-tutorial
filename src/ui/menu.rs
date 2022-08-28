use crate::data::AppData;
use druid::{Env, Menu, WindowId};

pub fn menu(_: Option<WindowId>, _state: &AppData, _: &Env) -> Menu<AppData> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = druid::platform_menus::mac::menu_bar();
    }
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "openbsd"))]
    {
        base = base.entry(druid::platform_menus::win::file::default());
    }
    // base.rebuild_on(|old_data, data, _env| old_data.menu_count != data.menu_count)
    base
}

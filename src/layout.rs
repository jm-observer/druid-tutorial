//! This example shows how to construct a basic layout,
//! using columns, rows, and loops, for repeated Widgets.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use crate::connection::tabs::build_tab_widget;
use crate::data::{AppData, TabConfig};
use crate::list_connection::list_connection;
use druid::widget::{AspectRatioBox, Button, Flex, Label, LineBreaking, ViewSwitcher};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

pub fn build_app() -> impl Widget<AppData> {
    let vs = ViewSwitcher::new(
        |app_s: &AppData, _| app_s.tab_config.clone(),
        |tc: &TabConfig, _, _| Box::new(build_tab_widget(tc)),
    );

    let mut col = Flex::row();
    // let mut col = Flex::row().with_child(list_connection());
    col.add_flex_child(list_connection(), 1.0);
    col.add_flex_child(vs, 1.0);
    col.debug_paint_layout()
}

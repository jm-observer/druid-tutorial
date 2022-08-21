use crate::data::AppData;
use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::widget::{Axis, Label, Tabs, TabsEdge, TabsTransition};
use druid::{Widget, WidgetExt};

pub fn init_tabs() -> impl Widget<AppData> {
    Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .with_tab("remote", init())
        .with_tab("local", init())
        .fix_width(600.0)
        .fix_height(700.0)
}

pub fn init() -> impl Widget<AppData> {
    Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_tab("content", init_content())
        .with_tab("connection", init_connection())
}

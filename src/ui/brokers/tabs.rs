use crate::data::{AppData, NumberedTabs};
use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::widget::{Axis, Tabs, TabsEdge, TabsTransition};
use druid::{Widget, WidgetExt};

pub fn init_tabs() -> impl Widget<AppData> {
    Tabs::for_policy(NumberedTabs)
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .fix_width(600.0)
        .fix_height(700.0)
        .lens(AppData::tabs)
}

// pub fn init() -> impl Widget<AppData> {
//     let tabs = Tabs::new()
//         .with_axis(Axis::Horizontal)
//         .with_edge(TabsEdge::Leading)
//         .with_tab("content", init_content())
//         .with_tab("connection", init_connection())
//         // .lens(AppData::tabs::tab_labels)
//     ;
//     tabs
// }

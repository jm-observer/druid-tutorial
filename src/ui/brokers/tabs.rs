use crate::data::common::impl_tab::NumberedTabs;
use crate::data::hierarchy::AppData;
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

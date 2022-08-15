use druid::widget::{Axis, Tabs, TabsEdge};

pub fn init_tabs() {
    Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_tab("remote", "");
}

pub fn init() {
    Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_tab("content", "")
        .with_tab("connection", "");
}

use crate::data::hierarchy::AppData;
use crate::ui::brokers::list::init_connect;
use crate::ui::brokers::tabs::init_tabs;
use druid::widget::{Container, Padding, Split};
use druid::{Color, Widget};

pub fn init_layout() -> impl Widget<AppData> {
    Padding::new(
        5.0,
        Container::new(
            Split::columns(init_connect(), init_tabs())
                .split_point(0.25)
                .draggable(true),
        )
        .border(Color::WHITE, 1.0),
    )
}

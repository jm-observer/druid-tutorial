use crate::data::AppData;
use crate::ui::brokers::list::init_connect;
use crate::ui::brokers::tabs::init_tabs;
use druid::widget::{Container, Padding, Split};
use druid::{Color, Widget, WidgetExt};

pub fn init_layout() -> impl Widget<AppData> {
    Padding::new(
        5.0,
        Container::new(
            Split::columns(init_connect(), init_tabs())
                .split_point(0.25)
                // .bar_size(5.0)
                // .min_bar_area(11.0)
                .draggable(true),
        )
        .border(Color::WHITE, 1.0),
    )
}

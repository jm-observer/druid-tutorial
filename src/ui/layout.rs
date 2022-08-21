use crate::data::AppData;
use crate::ui::brokers::list::init_connect;
use crate::ui::brokers::tabs::{init, init_tabs};
use druid::widget::{Container, CrossAxisAlignment, Flex, MainAxisAlignment, Padding, Split};
use druid::{Color, Widget, WidgetExt};

pub fn init_layout() -> impl Widget<AppData> {
    // let layout = Flex::row()
    //     .with_flex_child(init_connect(), 1.0)
    //     // .with_default_spacer()
    //     .with_flex_child(init_tabs(), 1.0)
    //     .cross_axis_alignment(CrossAxisAlignment::Start)
    //     .main_axis_alignment(MainAxisAlignment::Start);
    // let layout = layout.fix_width(1000.0).fix_height(800.0);
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

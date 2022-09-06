use crate::data::common::brokers::BrokerTab;
use druid::widget::{Align, Container, Flex, Label, Padding, Split};
use druid::{Color, Widget, WidgetExt};

pub fn init_content() -> Container<BrokerTab> {
    let topic_type = Padding::new(
        10.0,
        Container::new(
            Split::rows(
                Align::centered(Label::new("topic subscribe list")),
                Align::centered(Label::new("topic history list")),
            )
            .split_point(0.75)
            .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );
    let topic = Padding::new(
        10.0,
        Container::new(
            Split::rows(topic_type, Align::centered(Label::new("topic subscribe")))
                .split_point(0.75)
                .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );

    let msg = Padding::new(
        10.0,
        Container::new(
            Split::rows(
                Align::centered(Label::new("msg list")),
                Align::centered(Label::new("msg send")),
            )
            .split_point(0.75)
            .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );
    Container::new(Split::columns(topic, msg).split_point(0.2).draggable(true))
    // .debug_paint_layout()
}

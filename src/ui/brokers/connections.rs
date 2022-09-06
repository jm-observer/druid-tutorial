use crate::data::common::brokers::BrokerTab;
use druid::widget::{Container, Flex, Label, TextBox};
use druid::{Widget, WidgetExt};

//
pub fn init_connection() -> Container<BrokerTab> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("client id"))
                .with_child(TextBox::new().lens(BrokerTab::name)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("addr"))
                .with_child(TextBox::new().lens(BrokerTab::name)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("port"))
                .with_child(TextBox::new().lens(BrokerTab::name)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("params"))
                .with_child(TextBox::new().lens(BrokerTab::name)),
        );
    Container::new(connection)
}

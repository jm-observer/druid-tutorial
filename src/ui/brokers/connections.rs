use crate::data::{AppData, BrokerTab};
use druid::widget::{Flex, Label, TextBox};
use druid::{Widget, WidgetExt};

//
pub fn init_connection() -> Flex<BrokerTab> {
    // client_id/url/port/params
    Flex::column()
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
        )
}

use crate::data::AppData;
use druid::widget::{Flex, Label, TextBox};
use druid::{Widget, WidgetExt};

//
pub fn init_connection() -> impl Widget<AppData> {
    // client_id/url/port/params
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("client id"))
                .with_child(TextBox::new().lens(AppData::text)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("addr"))
                .with_child(TextBox::new().lens(AppData::text)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("port"))
                .with_child(TextBox::new().lens(AppData::text)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("params"))
                .with_child(TextBox::new().lens(AppData::text)),
        )
}

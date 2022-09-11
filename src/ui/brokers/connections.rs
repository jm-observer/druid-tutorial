use crate::data::hierarchy::broker_detail::BrokerTab;
use crate::data::AppEvent;
use druid::widget::{Container, Flex, Label, TextBox};
use druid::{Widget, WidgetExt};
use log::{debug, error};

//
pub fn init_connection() -> Container<BrokerTab> {
    let connection = Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("name").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerTab::name)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("client id").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerTab::client_id)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("addr").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerTab::addr)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("port").fix_width(80.0))
                .with_child(TextBox::new().lens(BrokerTab::port)),
        )
        .with_child(
            Flex::row().with_child(Label::new("连接").with_text_size(12.).on_click(
                move |_ctx, data: &mut BrokerTab, _env| {
                    data.is_try_connect = true;
                    debug!("{:?}", std::thread::current().name());
                    if let Err(e) = data.db.tx.send(AppEvent::Connect(data.id.clone())) {
                        error!("{:?}", e);
                    }
                    debug!("连接: {:?}", data);
                },
            )),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("params").fix_width(50.0))
                .with_flex_child(
                    TextBox::multiline()
                        .with_placeholder("Multi")
                        .lens(BrokerTab::params)
                        .fix_height(100.)
                        .expand_width(),
                    1.0,
                ),
        );
    Container::new(connection)
}

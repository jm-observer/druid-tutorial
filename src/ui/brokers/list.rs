use crate::data::common::brokers::Broker;
use crate::data::hierarchy::broker_detail::{BrokerTab, DynamicTabData};
use crate::data::hierarchy::AppData;
use druid::im::Vector;
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll};
use druid::{theme, Env};
use druid::{UnitPoint, WidgetExt};
use log::debug;

pub fn init_connect() -> Flex<AppData> {
    let name = || {
        Label::dynamic(|data: &Broker, _: &Env| format!("{}", data.name))
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .fix_width(120f64)
    };
    let addr = || {
        Label::dynamic(|data: &Broker, _: &Env| format!("{}:{}", data.addr, data.port))
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .fix_width(120f64)
    };

    let list: List<Broker> =
        List::new(move || Flex::row().with_flex_child(name(), 1.0).with_child(addr()));

    let scroll = Scroll::<Vector<Broker>, List<Broker>>::new(list);

    let buttons = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Label::new("新增")
                .with_text_size(12.)
                // .border(Color::WHITE, 10.)
                .on_click(move |_ctx, data: &mut DynamicTabData, _env| {
                    debug!("AAAAAAAA");
                    let tab = BrokerTab::default();
                    data.tab_labels.insert(tab.id.clone(), tab);
                }),
        )
        .with_child(Button::new("删"))
        .with_child(Button::new("复制"))
        .background(theme::PLACEHOLDER_COLOR)
        .lens(AppData::tabs);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(buttons).with_child(
        scroll
            .vertical()
            .lens(AppData::brokers)
            .fix_height(200.0)
            .fix_width(300.0),
    );
    flex
}

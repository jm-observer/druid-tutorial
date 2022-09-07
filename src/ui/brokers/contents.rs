use crate::data::common::publics::PublicMsgInput;
use crate::data::common::subscribes::{SubscribeHis, SubscribeInput, SubscribeTopic};
use crate::data::hierarchy::broker_detail::{
    BrokerTab, Msg, Msgs, SubscribeHises, SubscribeTopics,
};
use druid::im::Vector;
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Flex, Label, List, Padding, Scroll, Split,
    TextBox,
};
use druid::LensExt;
use druid::{Color, Env, UnitPoint, Widget, WidgetExt};
use log::debug;

pub fn init_content() -> Container<BrokerTab> {
    let topic_type = Padding::new(
        10.0,
        Container::new(
            Split::rows(
                Align::centered(init_subscribe_list()),
                Align::centered(init_subscribe_his_list()),
            )
            .split_point(0.75)
            .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );
    let topic = Padding::new(
        10.0,
        Container::new(
            Split::rows(topic_type, Align::centered(init_subscribe_input()))
                .split_point(0.75)
                .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );

    let msg = Padding::new(
        10.0,
        Container::new(
            Split::rows(
                Align::centered(init_msgs_list()),
                Align::centered(init_public_input()),
            )
            .split_point(0.75)
            .bar_size(3.0),
        )
        .border(Color::WHITE, 1.0),
    );
    Container::new(Split::columns(topic, msg).split_point(0.2).draggable(true))
    // .debug_paint_layout()
}

fn init_subscribe_list() -> impl Widget<BrokerTab> {
    let topic = || {
        Label::dynamic(|data: &SubscribeTopic, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic)
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &SubscribeTopic, _: &Env| format!("{:?}", data.qos))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };
    let status = || {
        Label::dynamic(|data: &SubscribeTopic, _: &Env| format!("{:?}", data.status))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<SubscribeTopic> = List::new(move || {
        Flex::row()
            .with_flex_child(topic(), 1.0)
            .with_child(qos())
            .with_child(status())
    });

    let scroll = Scroll::<Vector<SubscribeTopic>, List<SubscribeTopic>>::new(list);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(
        scroll
            .vertical()
            .lens(BrokerTab::subscribe_topics.then(SubscribeTopics::topics))
            .fix_height(200.0)
            .fix_width(300.0),
    );
    flex
}

fn init_msgs_list() -> impl Widget<BrokerTab> {
    let topic = || {
        Label::dynamic(|data: &Msg, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic())
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &Msg, _: &Env| format!("{:?}", data.qos()))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };
    let msg = || {
        Label::dynamic(|data: &Msg, _: &Env| format!("{:?}", data.msg()))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<Msg> = List::new(move || {
        Flex::row()
            .with_flex_child(topic(), 1.0)
            .with_child(qos())
            .with_child(msg())
    });

    let scroll = Scroll::<Vector<Msg>, List<Msg>>::new(list);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(
        scroll
            .vertical()
            .lens(BrokerTab::msgs.then(Msgs::msgs))
            .fix_height(200.0)
            .fix_width(300.0),
    );
    flex
}

fn init_subscribe_his_list() -> impl Widget<BrokerTab> {
    let topic = || {
        Label::dynamic(|data: &SubscribeHis, _: &Env| {
            debug!("{:?}", data);
            format!("{}", data.topic)
        })
        .align_vertical(UnitPoint::LEFT)
        .fix_width(20f64)
    };
    let qos = || {
        Label::dynamic(|data: &SubscribeHis, _: &Env| format!("{:?}", data.qos))
            .align_vertical(UnitPoint::LEFT)
            .fix_width(20f64)
    };

    let list: List<SubscribeHis> =
        List::new(move || Flex::row().with_flex_child(topic(), 1.0).with_child(qos()));

    let scroll = Scroll::<Vector<SubscribeHis>, List<SubscribeHis>>::new(list);

    let flex = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let flex = flex.with_child(
        scroll
            .vertical()
            .lens(BrokerTab::subscribe_hises.then(SubscribeHises::topics))
            .fix_height(200.0)
            .fix_width(300.0),
    );
    flex
}

//
pub fn init_subscribe_input() -> Container<BrokerTab> {
    let connection = Flex::row()
        .with_child(
            Flex::column()
                .with_child(Label::new("topic").fix_width(80.0))
                .with_child(Label::new("qos").fix_width(80.0)),
        )
        .with_child(
            Flex::column()
                .with_child(
                    TextBox::new().lens(BrokerTab::subscribe_ing.then(SubscribeInput::topic)),
                )
                .with_child(TextBox::new().lens(BrokerTab::subscribe_ing.then(SubscribeInput::qos)))
                .with_child(Button::new("订阅")),
        );
    Container::new(connection)
}

pub fn init_public_input() -> Container<BrokerTab> {
    let connection = Flex::row()
        .with_child(
            Flex::column()
                .with_child(Label::new("topic").fix_width(80.0))
                .with_child(Label::new("qos").fix_width(80.0))
                .with_child(Label::new("msg").fix_width(80.0)),
        )
        .with_child(
            Flex::column()
                .with_child(TextBox::new().lens(BrokerTab::public_ing.then(PublicMsgInput::topic)))
                .with_child(TextBox::new().lens(BrokerTab::public_ing.then(PublicMsgInput::qos)))
                .with_child(TextBox::new().lens(BrokerTab::public_ing.then(PublicMsgInput::msg)))
                .with_child(Button::new("发布")),
        );
    Container::new(connection)
}

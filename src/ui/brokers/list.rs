use crate::data::{AppData, Broker};
use druid::im::Vector;
use druid::lens::{Identity, Map, Then};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, LensWrap, List, Scroll, SizedBox};
use druid::{lens, Color, UnitPoint, WidgetExt};
use druid::{Env, LensExt};

pub fn init() {}

pub fn init_connect() -> Flex<AppData> {
    let name = || {
        Label::dynamic(|data: &Broker, _: &Env| format!("{}", data.name))
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .fix_width(120f64)
            .background(Color::rgb(0.5, 0.5, 0.5))
    };
    let addr = || {
        Label::dynamic(|data: &Broker, _: &Env| format!("{}:{}", data.addr, data.port))
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb(0.5, 0.5, 0.5))
            .fix_width(120f64)
    };

    let list: List<Broker> =
        List::new(move || Flex::row().with_flex_child(name(), 1.0).with_child(addr()));

    let scroll = Scroll::<Vector<Broker>, List<Broker>>::new(list);

    let flex = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);
    flex.with_child(scroll.vertical().lens(AppData::brokers).fix_height(200.0))
}

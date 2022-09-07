use crate::data::common::subscribes::SubscribeHis;
use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::im::{HashMap, Vector};
use druid::widget::{Flex, Label, TabInfo, TabsPolicy};
use druid::Lens;
use druid::{Data, Widget};

#[derive(Debug, Clone, Data, Lens)]
pub struct Broker {
    pub id: String,
    pub(crate) name: String,
    pub(crate) addr: String,
    pub(crate) port: u16,
    pub subscribe_his: Vector<SubscribeHis>,
}

#[derive(Data, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum TabKind {
    Connections,
    Content,
}
#[derive(Data, Debug, Clone, Eq, PartialEq)]
pub enum Qos {
    Qos0,
    Qos1,
    Qos2,
}
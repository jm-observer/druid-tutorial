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
#[derive(Debug, Clone, Data, Lens, Eq)]
pub struct BrokerTab {
    pub id: String,
    pub is_store: bool,
    pub is_try_connect: bool,
    pub(crate) name: String,
    pub(crate) addr: String,
    pub(crate) port: u16,
}

#[derive(Data, Clone, Lens, Debug, Hash, Eq, PartialEq)]
pub struct DynamicTabData {
    pub tab_labels: WrapHashMap,
}

#[derive(Data, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WrapHashMap(pub HashMap<String, BrokerTab>);

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

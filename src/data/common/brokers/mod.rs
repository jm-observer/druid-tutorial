use crate::data::common::subscribes::SubscribeHis;
use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::im::{HashMap, Vector};
use druid::widget::{Flex, Label, TabInfo, TabsPolicy};
use druid::Lens;
use druid::{Data, Widget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Data, Lens, Serialize, Deserialize)]
pub struct Broker {
    pub id: String,
    pub name: String,
    pub addr: String,
    pub port: String,
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

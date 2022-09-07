use crate::data::common::brokers::Qos;
use crate::data::common::publics::{PublicMsg, PublicMsgInput};
use crate::data::common::subscribes::{SubscribeHis, SubscribeInput, SubscribeMsg, SubscribeTopic};
use druid::im::{HashMap, Vector};
use druid::Data;
use druid::Lens;

#[derive(Data, Clone, Lens, Debug, Hash, Eq, PartialEq)]
pub struct DynamicTabData {
    pub tab_labels: WrapHashMap,
}

#[derive(Data, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WrapHashMap(pub HashMap<String, BrokerTab>);

#[derive(Debug, Clone, Data, Lens, Eq)]
pub struct BrokerTab {
    pub id: String,
    pub is_store: bool,
    pub is_try_connect: bool,
    pub(crate) client_id: String,
    pub(crate) name: String,
    pub(crate) addr: String,
    pub params: String,
    pub(crate) port: String,
    pub subscribe_topics: SubscribeTopics,
    pub subscribe_hises: SubscribeHises,
    pub msgs: Msgs,
    pub subscribe_ing: SubscribeInput,
    pub public_ing: PublicMsgInput,
}

#[derive(Debug, Data, Clone, Eq, PartialEq, Default, Lens)]
pub struct SubscribeTopics {
    pub topics: Vector<SubscribeTopic>,
}
#[derive(Debug, Data, Clone, Eq, PartialEq, Default, Lens)]
pub struct SubscribeHises {
    pub topics: Vector<SubscribeHis>,
}

#[derive(Debug, Data, Clone, Eq, PartialEq, Default, Lens)]
pub struct Msgs {
    pub msgs: Vector<Msg>,
}

#[derive(Debug, Data, Clone, Eq, PartialEq)]
pub enum Msg {
    Public(PublicMsg),
    Subscribe(SubscribeMsg),
}

impl Msg {
    pub fn qos(&self) -> &Qos {
        match self {
            Msg::Subscribe(msg) => &msg.qos,
            Msg::Public(msg) => &msg.qos,
        }
    }
    pub fn msg(&self) -> &String {
        match self {
            Msg::Subscribe(msg) => &msg.msg,
            Msg::Public(msg) => &msg.msg,
        }
    }
    pub fn topic(&self) -> &String {
        match self {
            Msg::Subscribe(msg) => &msg.topic,
            Msg::Public(msg) => &msg.topic,
        }
    }
}

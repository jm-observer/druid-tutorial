use crate::data::common::brokers::{Broker, Qos};
use crate::data::common::publics::{PublicMsg, PublicMsgInput};
use crate::data::common::subscribes::{SubscribeHis, SubscribeInput, SubscribeMsg, SubscribeTopic};
use crate::data::db::ArcDb;
use crate::data::AString;
use druid::im::{HashMap, Vector};
use druid::Data;
use druid::Lens;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Data, Clone, Lens, Debug)]
pub struct DynamicTabData {
    pub tab_labels: WrapHashMap,
    #[data(ignore)]
    #[lens(ignore)]
    pub db: ArcDb,
}

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct WrapHashMap(pub HashMap<AString, BrokerTab>);

#[derive(Debug, Clone, Data, Lens)]
pub struct BrokerTab {
    pub id: AString,
    pub is_store: bool,
    pub is_try_connect: bool,
    pub client_id: AString,
    pub name: AString,
    pub addr: AString,
    pub params: AString,
    pub port: AString,
    pub subscribe_topics: SubscribeTopics,
    pub subscribe_hises: SubscribeHises,
    pub msgs: Msgs,
    pub subscribe_ing: SubscribeInput,
    pub public_ing: PublicMsgInput,
    #[data(ignore)]
    pub db: ArcDb,
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
    pub fn msg(&self) -> &AString {
        match self {
            Msg::Subscribe(msg) => &msg.msg,
            Msg::Public(msg) => &msg.msg,
        }
    }
    pub fn topic(&self) -> &AString {
        match self {
            Msg::Subscribe(msg) => &msg.topic,
            Msg::Public(msg) => &msg.topic,
        }
    }
}

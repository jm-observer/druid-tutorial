use crate::data::common::brokers::Qos;
use crate::data::AString;
use druid::{Data, Lens};

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct SubscribeMsg {
    pub topic: AString,
    pub msg: AString,
    pub qos: Qos,
}

#[derive(Data, Clone, Debug, Eq, PartialEq)]
pub struct SubscribeTopic {
    pub topic: AString,
    pub qos: Qos,
    pub status: SubscribeStatus,
}
#[derive(Data, Debug, Clone, Eq, PartialEq, Lens)]
pub struct SubscribeHis {
    pub(crate) topic: AString,
    pub(crate) qos: Qos,
}
#[derive(Data, Debug, Clone, Eq, PartialEq)]
pub enum SubscribeStatus {
    Ing,
    Success,
    Fail,
}

#[derive(Data, Debug, Clone, Eq, PartialEq, Lens)]
pub struct SubscribeInput {
    pub(crate) topic: AString,
    pub(crate) qos: AString,
}

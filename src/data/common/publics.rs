use crate::data::common::brokers::Qos;
use crate::data::AString;
use druid::{Data, Lens};

#[derive(Debug, Data, Clone, Eq, PartialEq, Lens)]
pub struct PublicMsg {
    pub topic: AString,
    pub msg: AString,
    pub qos: Qos,
    pub status: PublicStatus,
}
#[derive(Debug, Data, Clone, Eq, PartialEq)]
pub enum PublicStatus {
    Ing,
    Success,
}

#[derive(Debug, Data, Clone, Eq, PartialEq, Lens)]
pub struct PublicMsgInput {
    pub topic: AString,
    pub msg: AString,
    pub qos: AString,
}

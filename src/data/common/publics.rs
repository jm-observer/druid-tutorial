use crate::data::common::brokers::Qos;
use druid::{Data, Lens};
#[derive(Debug, Data, Clone, Eq, PartialEq, Lens)]
pub struct PublicMsg {
    pub topic: String,
    pub msg: String,
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
    pub topic: String,
    pub msg: String,
    pub qos: String,
}

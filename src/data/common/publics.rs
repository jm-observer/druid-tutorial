use crate::data::common::brokers::Qos;
use druid::Data;
#[derive(Data, Clone)]
pub struct PublicMsg {
    pub topic: String,
    pub msg: String,
    pub qos: Qos,
    pub status: PublicStatus,
}
#[derive(Data, Clone, Eq, PartialEq)]
pub enum PublicStatus {
    Ing,
    Success,
}

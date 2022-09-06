use crate::data::common::brokers::Qos;
use druid::Data;

#[derive(Data, Clone)]
pub struct SubscribeMsg {
    topic: String,
    msg: String,
    qos: Qos,
}

#[derive(Data, Clone)]
pub struct SubscribeTopic {
    topic: String,
    qos: Qos,
    status: SubscribeStatus,
}
#[derive(Data, Debug, Clone)]
pub struct SubscribeHis {
    topic: String,
    qos: Qos,
}
#[derive(Data, Debug, Clone, Eq, PartialEq)]
pub enum SubscribeStatus {
    Ing,
    Success,
    Fail,
}

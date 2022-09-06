use crate::data::common::subscribes::SubscribeTopic;
use druid::im::Vector;
use druid::Data;

#[derive(Data, Clone)]
pub struct SubscribeTopics {
    topics: Vector<SubscribeTopic>,
}

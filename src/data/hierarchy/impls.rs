use crate::data::common::brokers::Broker;
use crate::data::hierarchy::broker_detail::{
    BrokerTab, DynamicTabData, Msgs, SubscribeHises, SubscribeTopics, WrapHashMap,
};
use crate::data::hierarchy::AppData;
use druid::im::{vector, HashMap};
use std::hash::{Hash, Hasher};
use std::ops;

impl PartialEq for BrokerTab {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for BrokerTab {}
impl Hash for BrokerTab {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for DynamicTabData {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.tab_labels, &other.tab_labels)
    }
}
impl Eq for DynamicTabData {}

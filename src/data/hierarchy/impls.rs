use crate::data::common::brokers::Broker;
use crate::data::hierarchy::broker_detail::{
    BrokerTab, DynamicTabData, Msgs, SubscribeHises, SubscribeTopics, WrapHashMap,
};
use crate::data::hierarchy::AppData;
use crate::mock::{mock_msgs, mock_subscribe_hises, mock_subscribe_topics};
use druid::im::{vector, HashMap};
use std::hash::{Hash, Hasher};
use std::ops;

impl PartialEq for BrokerTab {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for BrokerTab {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Default for BrokerTab {
    fn default() -> Self {
        // todo
        Self {
            id: "123".to_string(),
            is_store: false,
            is_try_connect: false,
            client_id: "id123".to_string(),
            name: "".to_string(),
            addr: "".to_string(),
            params: "{}".to_string(),
            port: "0".to_string(),
            subscribe_topics: mock_subscribe_topics(),
            subscribe_hises: mock_subscribe_hises(),
            msgs: mock_msgs(),
            subscribe_ing: Default::default(),
            public_ing: Default::default(),
        }
    }
}

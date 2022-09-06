use crate::data::common::brokers::{Broker, BrokerTab, DynamicTabData, WrapHashMap};
use crate::data::hierarchy::AppData;
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

impl Default for AppData {
    fn default() -> Self {
        let b0 = Broker {
            id: "1".to_string(),
            name: "local".to_string(),
            addr: "127.0.0.1".to_string(),
            port: 8080,
            subscribe_his: Default::default(),
        };
        let b1 = Broker {
            id: "2".to_string(),
            name: "remote".to_string(),
            addr: "192.168.32.152".to_string(),
            port: 8000,
            subscribe_his: Default::default(),
        };
        let brokers = vector![b0, b1];
        Self {
            brokers,
            tabs: DynamicTabData {
                tab_labels: WrapHashMap(HashMap::default()),
            },
            text: "".to_string(),
            tabs_size: 0,
        }
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            addr: "".to_string(),
            port: 0,
            subscribe_his: Default::default(),
        }
    }
}
impl Default for BrokerTab {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            is_store: false,
            is_try_connect: false,
            name: "".to_string(),
            addr: "".to_string(),
            port: 0,
        }
    }
}
impl From<Broker> for BrokerTab {
    fn from(val: Broker) -> Self {
        let Broker {
            id,
            name,
            addr,
            port,
            ..
        } = val;
        Self {
            id,
            name,
            addr,
            port,
            is_store: true,
            is_try_connect: false,
        }
    }
}

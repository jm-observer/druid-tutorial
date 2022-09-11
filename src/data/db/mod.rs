use anyhow::Result;
use sled::{Config, Db};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use crate::data::common::brokers::{Broker, Qos};
use crate::data::common::publics::{PublicMsg, PublicStatus};
use crate::data::common::subscribes::{
    SubscribeHis, SubscribeMsg, SubscribeStatus, SubscribeTopic,
};
use crate::data::hierarchy::broker_detail::{
    BrokerTab, DynamicTabData, Msg, Msgs, SubscribeHises, SubscribeTopics, WrapHashMap,
};
use crate::data::hierarchy::AppData;
use crate::data::AppEvent;
use druid::im::{HashMap, Vector};

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub db: Arc<Db>,
    pub tx: Sender<AppEvent>,
}

const BROKERS: &[u8; 7] = b"brokers";
const KEYS: &[u8; 4] = b"keys";
impl ArcDb {
    pub fn init_db(tx: Sender<AppEvent>) -> Result<Self> {
        let config = Config::new();
        Ok(ArcDb {
            db: Arc::new(config.open()?),
            tx,
        })
    }
    pub fn read_app_data(&self) -> Result<AppData> {
        Ok(AppData {
            brokers: self.read_brokers()?,
            tabs: DynamicTabData {
                tab_labels: WrapHashMap(HashMap::default()),
                db: self.clone(),
            },
            db: self.clone(),
        })
    }
    pub fn read_brokers(&self) -> Result<Vector<Broker>> {
        if let Some(val) = self.db.get(BROKERS)? {
            let brokers: Vector<Broker> = serde_json::from_slice(&val)?;
            Ok(brokers)
        } else {
            Ok(Vector::new())
        }
    }
    pub fn new_broker_tab(&self) -> BrokerTab {
        BrokerTab {
            id: uuid::Uuid::new_v4().to_string().into(),
            is_store: false,
            is_try_connect: false,
            client_id: "".to_string().into(),
            name: "".to_string().into(),
            addr: "".to_string().into(),
            params: "".to_string().into(),
            port: "".to_string().into(),
            subscribe_topics: Default::default(),
            subscribe_hises: Default::default(),
            msgs: Default::default(),
            subscribe_ing: Default::default(),
            public_ing: Default::default(),
            db: self.clone(),
        }
    }
    pub fn save_broker(&self, tab: BrokerTab) {
        let broker = Broker {
            id: tab.id.clone(),
            name: tab.name.clone(),
            addr: tab.addr.clone(),
            port: tab.port.clone(),
        };
    }
}
//
// pub fn read_subscribe_his(id: &str) -> Vector<SubscribeHis> {
//     let mut brokers = Vector::new();
//     brokers.push_back(SubscribeHis {
//         topic: "/abc/".to_string(),
//         qos: Qos::Qos0,
//     });
//     brokers
// }
//
// pub fn mock_subscribe_topics() -> SubscribeTopics {
//     let mut topics = Vector::new();
//     topics.push_back(SubscribeTopic {
//         topic: "/a/1".to_string(),
//         qos: Qos::Qos0,
//         status: SubscribeStatus::Ing,
//     });
//     topics.push_back(SubscribeTopic {
//         topic: "/b/2".to_string(),
//         qos: Qos::Qos1,
//         status: SubscribeStatus::Success,
//     });
//     SubscribeTopics { topics }
// }
// pub fn mock_subscribe_hises() -> SubscribeHises {
//     let mut topics = Vector::new();
//     topics.push_back(SubscribeHis {
//         topic: "/a/1".to_string(),
//         qos: Qos::Qos0,
//     });
//     topics.push_back(SubscribeHis {
//         topic: "/b/2".to_string(),
//         qos: Qos::Qos1,
//     });
//     SubscribeHises { topics }
// }
// pub fn mock_msgs() -> Msgs {
//     let mut msgs = Vector::new();
//     msgs.push_back(Msg::Public(PublicMsg {
//         topic: "/a/2".to_string(),
//         msg: "{'a': 12}".to_string(),
//         qos: Qos::Qos0,
//         status: PublicStatus::Ing,
//     }));
//     msgs.push_back(Msg::Subscribe(SubscribeMsg {
//         topic: "/a/1".to_string(),
//         msg: "{'a': 12}".to_string(),
//         qos: Qos::Qos0,
//     }));
//     msgs.push_back(Msg::Public(PublicMsg {
//         topic: "/a/2".to_string(),
//         msg: "1111".to_string(),
//         qos: Qos::Qos1,
//         status: PublicStatus::Ing,
//     }));
//     Msgs { msgs }
// }

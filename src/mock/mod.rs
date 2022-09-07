use crate::data::common::brokers::{Broker, Qos};
use crate::data::common::publics::{PublicMsg, PublicStatus};
use crate::data::common::subscribes::{
    SubscribeHis, SubscribeMsg, SubscribeStatus, SubscribeTopic,
};
use crate::data::hierarchy::broker_detail::{
    DynamicTabData, Msg, Msgs, SubscribeHises, SubscribeTopics, WrapHashMap,
};
use crate::data::hierarchy::AppData;
use druid::im::{HashMap, Vector};

pub fn read_app_data() -> AppData {
    AppData {
        brokers: read_brokers(),
        tabs: DynamicTabData {
            tab_labels: WrapHashMap(HashMap::default()),
        },
    }
}

pub fn read_brokers() -> Vector<Broker> {
    let mut brokers = Vector::new();
    brokers.push_back(Broker {
        id: "1234".to_string(),
        name: "emq".to_string(),
        addr: "192.168.199.10".to_string(),
        port: 8080,
        subscribe_his: read_subscribe_his(""),
    });
    brokers
}

pub fn read_subscribe_his(id: &str) -> Vector<SubscribeHis> {
    let mut brokers = Vector::new();
    brokers.push_back(SubscribeHis {
        topic: "/abc/".to_string(),
        qos: Qos::Qos0,
    });
    brokers
}

pub fn mock_subscribe_topics() -> SubscribeTopics {
    let mut topics = Vector::new();
    topics.push_back(SubscribeTopic {
        topic: "/a/1".to_string(),
        qos: Qos::Qos0,
        status: SubscribeStatus::Ing,
    });
    topics.push_back(SubscribeTopic {
        topic: "/b/2".to_string(),
        qos: Qos::Qos1,
        status: SubscribeStatus::Success,
    });
    SubscribeTopics { topics }
}
pub fn mock_subscribe_hises() -> SubscribeHises {
    let mut topics = Vector::new();
    topics.push_back(SubscribeHis {
        topic: "/a/1".to_string(),
        qos: Qos::Qos0,
    });
    topics.push_back(SubscribeHis {
        topic: "/b/2".to_string(),
        qos: Qos::Qos1,
    });
    SubscribeHises { topics }
}
pub fn mock_msgs() -> Msgs {
    let mut msgs = Vector::new();
    msgs.push_back(Msg::Public(PublicMsg {
        topic: "/a/2".to_string(),
        msg: "{'a': 12}".to_string(),
        qos: Qos::Qos0,
        status: PublicStatus::Ing,
    }));
    msgs.push_back(Msg::Subscribe(SubscribeMsg {
        topic: "/a/1".to_string(),
        msg: "{'a': 12}".to_string(),
        qos: Qos::Qos0,
    }));
    msgs.push_back(Msg::Public(PublicMsg {
        topic: "/a/2".to_string(),
        msg: "1111".to_string(),
        qos: Qos::Qos1,
        status: PublicStatus::Ing,
    }));
    Msgs { msgs }
}

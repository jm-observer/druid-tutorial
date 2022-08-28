use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::im::{vector, HashMap, Vector};
use druid::widget::{Axis, Flex, Label, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition};
use druid::{Data, Lens, Widget, WidgetExt};
use log::warn;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
    pub brokers: Vector<Broker>,
    pub tabs: DynamicTabData,
    pub text: String,
    pub tabs_size: usize,
}
#[derive(Debug, Clone, Data, Lens)]
pub struct Broker {
    pub id: String,
    pub(crate) name: String,
    pub(crate) addr: String,
    pub(crate) port: u16,
}
#[derive(Debug, Clone, Data, Lens, Eq)]
pub struct BrokerTab {
    pub id: String,
    pub is_store: bool,
    pub is_try_connect: bool,
    pub(crate) name: String,
    pub(crate) addr: String,
    pub(crate) port: u16,
}

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
        };
        let b1 = Broker {
            id: "2".to_string(),
            name: "remote".to_string(),
            addr: "192.168.32.152".to_string(),
            port: 8000,
        };
        let brokers = vector![b0, b1];
        Self {
            brokers,
            tabs: DynamicTabData::new(),
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

#[derive(Clone, Data)]
pub struct NumberedTabs;
impl TabsPolicy for NumberedTabs {
    type Key = String;
    type Build = ();
    type Input = DynamicTabData;
    type LabelWidget = Label<DynamicTabData>;
    type BodyWidget = Label<DynamicTabData>;

    fn tabs_changed(&self, old_data: &DynamicTabData, data: &DynamicTabData) -> bool {
        old_data == data
    }

    fn tabs(&self, data: &DynamicTabData) -> Vec<Self::Key> {
        data.tab_labels
            .clone()
            .into_iter()
            .map(|(key, _)| key)
            .collect()
    }

    fn tab_info(&self, key: Self::Key, _data: &DynamicTabData) -> TabInfo<DynamicTabData> {
        TabInfo::new(format!("Tab {:?}", key), true)
    }

    fn tab_body(&self, key: Self::Key, _data: &DynamicTabData) -> Label<DynamicTabData> {
        // Tabs::for_policy(BrokerTab)
        //     .with_axis(Axis::Horizontal)
        //     .with_edge(TabsEdge::Leading)
        //     .with_transition(TabsTransition::Instant)
        //     .fix_width(600.0)
        //     .fix_height(700.0)
        //     .lens(AppData::tabs.index(key))
        todo!()
    }

    fn close_tab(&self, key: Self::Key, data: &mut DynamicTabData) {
        data.tab_labels.remove(&key);
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}

#[derive(Data, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum TabKind {
    Connections,
    Content,
}

impl TabsPolicy for BrokerTab {
    type Key = TabKind;
    type Build = ();
    type Input = BrokerTab;
    type LabelWidget = Label<BrokerTab>;
    type BodyWidget = Flex<BrokerTab>;

    fn tabs_changed(&self, old_data: &BrokerTab, data: &BrokerTab) -> bool {
        old_data.is_try_connect != data.is_try_connect
    }

    fn tabs(&self, data: &BrokerTab) -> Vec<Self::Key> {
        let mut keys = Vec::with_capacity(2);
        if data.is_try_connect {
            keys.push(TabKind::Connections)
        }
        keys.push(TabKind::Content);
        keys
    }

    fn tab_info(&self, key: Self::Key, _data: &BrokerTab) -> TabInfo<BrokerTab> {
        match key {
            TabKind::Connections => TabInfo::new(format!("Connection {:?}", _data.name), true),
            TabKind::Content => TabInfo::new(format!("Content {:?}", _data.name), false),
        }
    }

    fn tab_body(&self, key: Self::Key, _data: &BrokerTab) -> Flex<BrokerTab> {
        match key {
            TabKind::Connections => init_connection(),
            TabKind::Content => init_content(),
        }
    }

    fn close_tab(&self, key: Self::Key, data: &mut BrokerTab) {
        todo!()
        // if let Some(idx) = data.tab_labels.index_of(&key) {
        //     data.remove_tab(idx)
        // }
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}

#[derive(Data, Clone, Lens, Debug, Hash, Eq, PartialEq)]
pub struct DynamicTabData {
    // key: usize,
    // tab_labels: Vector<BrokerTab>,
    tab_labels: HashMap<String, BrokerTab>,
}

impl DynamicTabData {
    pub fn new() -> Self {
        DynamicTabData {
            tab_labels: Default::default(),
        }
    }

    pub fn add_tab(&mut self, tab: BrokerTab) {
        self.tab_labels.insert(tab.id.clone(), tab);
    }

    pub fn remove_tab(&mut self, idx: String) {
        self.tab_labels.remove(&idx);
    }
}

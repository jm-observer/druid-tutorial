mod broker;

use crate::data::broker::BrokerTabPolicy;
use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::im::{vector, HashMap, Vector};
use druid::lens::{Deref, Index, Then};
use druid::widget::{
    Axis, Flex, Label, LensWrap, SizedBox, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition,
};
use druid::{Data, Lens, WidgetExt};
use druid::{LensExt, Widget};
use log::debug;
use std::hash::{Hash, Hasher};
use std::ops;

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
    type BodyWidget = impl Widget<DynamicTabData>;

    fn tabs_changed(&self, old_data: &DynamicTabData, data: &DynamicTabData) -> bool {
        // debug!("*********: {}", old_data != data);
        old_data != data
    }

    fn tabs(&self, data: &DynamicTabData) -> Vec<Self::Key> {
        let tabs: Vec<String> = data
            .tab_labels
            .0
            .clone()
            .into_iter()
            .map(|(key, _)| key)
            .collect();
        debug!("tabs.len = {}", tabs.len());
        tabs
    }

    fn tab_info(&self, key: Self::Key, _data: &DynamicTabData) -> TabInfo<DynamicTabData> {
        TabInfo::new(format!("Tab {:?}", key), true)
    }

    fn tab_body(&self, _key: String, _data: &DynamicTabData) -> Self::BodyWidget {
        debug!("tab_body");
        Tabs::for_policy(BrokerTabPolicy)
            .with_axis(Axis::Horizontal)
            .with_edge(TabsEdge::Leading)
            .with_transition(TabsTransition::Instant)
            // .expand()
            // .fix_width(600.0)
            // .fix_height(700.0)
            .lens(DynamicTabData::tab_labels.index(_key))
        // .debug_paint_layout()
        // let body: Tabs<BrokerTabPolicy> = Tabs::for_policy(BrokerTabPolicy)
        //     .with_axis(Axis::Horizontal)
        //     .with_edge(TabsEdge::Leading)
        //     .with_transition(TabsTransition::Instant)
        //     ;
        // let index = Index::new(_key);
        // pub trait LensExt<A: ?Sized, B: ?Sized>: Lens<A, B> {
        // fn index<I>(self, index: I) -> Then<Self, Index<I>, B>
        // Lens<AppData, HashMap<String, BrokerTab>
        // Then<Self, Index<I>, B>
        // let first: impl Lens<AppData, HashMap<String, BrokerTab>> = AppData::tabs;
        // let data: Then<app_data_derived_lenses::tabs, Index<&String>, HashMap<String, BrokerTab>> =
        //     AppData::tabs.index(&_key);
        // // let then: Then<AppData, Index<String>, DynamicTabData> = Then::new(AppData::tabs, index);
        // let sized_box: SizedBox<BrokerTab> = body.fix_width(600.0).fix_height(700.0);

        // pub trait WidgetExt<T: Data>
        //     fn lens<S: Data, L: Lens<S, T>>(self, lens: L) -> LensWrap<S, T, L, Self> {
        //  impl<T, U, A, B, C> Lens<A, C> for Then<T, U, B>
        // let body: LensWrap<
        //     AppData,
        //     BrokerTab,
        //     Then<app_data_derived_lenses::tabs, Index<&String>, HashMap<String, BrokerTab>>,
        //     SizedBox<BrokerTab>,
        // > = sized_box.lens(data);
        // // LensWarp<AppData, BrokerTab, Then<app_data_derived_lenses::tabs, Index<&String>, HashMap<String, BrokerTab>>, SizedBox<BrokerTab>>
        // // // trait WidgetExt<T: Data>
        // // // fn lens<S: Data, L: Lens<S, T>>(self, lens: L) -> LensWrap<S, T, L, Self>
        // // let body = sized_box.lens(then);
        // body
        // todo!()
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

#[derive(Data, Clone, Lens, Debug, Hash, Eq, PartialEq)]
pub struct DynamicTabData {
    pub tab_labels: WrapHashMap,
}
#[derive(Data, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WrapHashMap(HashMap<String, BrokerTab>);

impl ops::Deref for WrapHashMap {
    type Target = HashMap<String, BrokerTab>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for WrapHashMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (self.0)
    }
}

impl ops::Index<String> for WrapHashMap {
    type Output = BrokerTab;

    fn index(&self, index: String) -> &Self::Output {
        self.0.index(&index)
    }
}
impl ops::IndexMut<String> for WrapHashMap {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        self.0.index_mut(&index)
    }
}

// pub type DynamicTabData = HashMap<String, BrokerTab>;

// impl DynamicTabData {
//     pub fn new() -> Self {
//         DynamicTabData {
//             tab_labels: Default::default(),
//         }
//     }
//
//     pub fn add_tab(&mut self, tab: BrokerTab) {
//         self.tab_labels.insert(tab.id.clone(), tab);
//     }
//
//     pub fn remove_tab(&mut self, idx: String) {
//         self.tab_labels.remove(&idx);
//     }
// }

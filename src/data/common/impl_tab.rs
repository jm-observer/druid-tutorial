use crate::data::common::brokers::{BrokerTab, DynamicTabData, TabKind};
use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::widget::{Axis, Label, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition};
use druid::Data;
use druid::LensExt;
use druid::{Widget, WidgetExt};
use log::debug;

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

#[derive(Data, Clone)]
pub struct BrokerTabPolicy;

impl TabsPolicy for BrokerTabPolicy {
    type Key = TabKind;
    type Build = ();
    type Input = BrokerTab;
    type LabelWidget = impl Widget<BrokerTab>;
    type BodyWidget = impl Widget<BrokerTab>;

    fn tabs_changed(&self, old_data: &BrokerTab, data: &BrokerTab) -> bool {
        old_data.is_try_connect != data.is_try_connect
    }

    fn tabs(&self, data: &BrokerTab) -> Vec<Self::Key> {
        let mut keys = Vec::with_capacity(2);
        // if data.is_try_connect {
        keys.push(TabKind::Connections);
        // }
        keys.push(TabKind::Content);
        keys
    }

    fn tab_info(&self, key: Self::Key, _data: &BrokerTab) -> TabInfo<BrokerTab> {
        match key {
            TabKind::Connections => TabInfo::new(format!("Connection {:?}", _data.name), true),
            TabKind::Content => TabInfo::new(format!("Content {:?}", _data.name), false),
        }
    }

    fn tab_body(&self, key: Self::Key, _data: &BrokerTab) -> Self::BodyWidget {
        match key {
            TabKind::Connections => init_content(),
            // TabKind::Content => init_connection(),
            TabKind::Content => init_connection(),
        }
        // init_content()
    }

    fn close_tab(&self, _key: Self::Key, _data: &mut BrokerTab) {
        // let index = Index::new(_key);
        // todo!()
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

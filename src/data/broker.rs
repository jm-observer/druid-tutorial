use crate::data::{BrokerTab, DynamicTabData};
use crate::ui::brokers::connections::init_connection;
use crate::ui::brokers::contents::init_content;
use druid::widget::{Flex, Label, TabInfo, TabsPolicy};
use druid::Data;

#[derive(Data, Clone)]
pub struct BrokerTabPolicy;

#[derive(Data, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum TabKind {
    Connections,
    Content,
}

impl TabsPolicy for BrokerTabPolicy {
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

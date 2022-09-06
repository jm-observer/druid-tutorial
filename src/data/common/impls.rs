use crate::data::common::brokers::{BrokerTab, WrapHashMap};
use druid::im::HashMap;
use std::ops;

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

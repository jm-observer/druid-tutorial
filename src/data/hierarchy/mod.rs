pub mod broker_detail;
pub mod connection;
pub mod impls;

use crate::data::common::brokers::Broker;
use crate::data::db::ArcDb;
use crate::data::hierarchy::broker_detail::DynamicTabData;
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
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::ops;
use std::rc::Rc;

#[derive(Debug, Clone, Lens, Data)]
pub struct AppData {
    pub brokers: Vector<Broker>,
    pub tabs: DynamicTabData,
    #[data(ignore)]
    #[lens(ignore)]
    pub db: ArcDb,
}

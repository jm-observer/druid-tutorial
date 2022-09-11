use std::sync::Arc;

pub mod common;
pub mod db;
pub mod hierarchy;

pub type AString = Arc<String>;

pub enum AppEvent {
    Connect(AString),
}

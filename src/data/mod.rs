pub mod common;
pub mod db;
pub mod hierarchy;

pub enum AppEvent {
    Connect(String),
}

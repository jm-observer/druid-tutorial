use druid::im::{vector, Vector};
use druid::{Data, Lens};

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
    pub brokers: Vector<Broker>,
    pub text: String,
}
#[derive(Debug, Clone, Data, Lens)]
pub struct Broker {
    pub(crate) name: String,
    pub(crate) addr: String,
    pub(crate) port: u16,
}

impl Default for AppData {
    fn default() -> Self {
        let b0 = Broker {
            name: "local".to_string(),
            addr: "127.0.0.1".to_string(),
            port: 8080,
        };
        let b1 = Broker {
            name: "remote".to_string(),
            addr: "192.168.32.152".to_string(),
            port: 8000,
        };
        let brokers = vector![b0, b1];
        Self {
            brokers,
            text: "".to_string(),
        }
    }
}

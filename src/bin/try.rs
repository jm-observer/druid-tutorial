use druid::{AppLauncher, PlatformError, WindowDesc};
use druid_tutorial::data::common::brokers::Broker;
use druid_tutorial::data::db::ArcDb;
use druid_tutorial::data::hierarchy::AppData;
use druid_tutorial::data::AppEvent;
use druid_tutorial::ui::layout::init_layout;
use druid_tutorial::ui::menu::menu;
use std::sync::mpsc::Receiver;
use std::thread;

fn main() -> Result<(), PlatformError> {
    custom_utils::logger::logger_stdout_debug();
    let win = WindowDesc::new(init_layout()).menu(menu);

    let (tx, rx) = std::sync::mpsc::channel();
    let db = ArcDb::init_db(tx)?;
    let data = db.read_app_data()?;

    let launcher = AppLauncher::with_window(win);
    let event_sink = launcher.get_external_handle();
    thread::spawn(move || generate_colors(event_sink, rx));

    launcher.launch(data)?;
    Ok(())
}

fn generate_colors(event_sink: druid::ExtEventSink, rx: Receiver<AppEvent>) {
    // to the main thread.
    loop {
        if let Ok(event) = rx.recv() {
            event_sink.add_idle_callback(move |data: &mut AppData| match event {
                AppEvent::Connect(id) => {
                    if let Some(tab) = data.tabs.tab_labels.get(&id) {
                        data.brokers.push_back(Broker {
                            id,
                            name: tab.name.clone(),
                            addr: tab.addr.clone(),
                            port: tab.port.clone(),
                        })
                    }
                }
            });
        } else {
            break;
        }
    }
}

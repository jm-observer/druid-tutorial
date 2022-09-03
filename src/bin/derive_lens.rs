use druid::im::HashMap;
use druid::Lens;
fn main() {}

#[derive(Lens)]
struct AppData {
    b: HashMap<String, String>,
}

use druid::im::HashMap;
use druid::widget::Slider;
use druid::widget::{CrossAxisAlignment, Flex, Label};
use druid::LensExt;
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

pub fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("lens-demo-window-title").with_placeholder("Lens Demo"));
    let mut b = HashMap::new();
    b.insert(true, 0.5);

    let data = A { b };

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<A> {
    let slider = Slider::new().lens(A::b.index(&true));
    let label = Label::new(|d: &A, _: &Env| format!("{:.2}", d.b.get(&true).unwrap()));
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(label)
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(slider),
        )
        .center()
}

#[derive(Data, Lens, Clone)]
struct A {
    b: HashMap<bool, f64>,
}
// #[derive(Data, Lens, Clone)]
// struct B {
//     c: HashMap<bool, f64>,
// }

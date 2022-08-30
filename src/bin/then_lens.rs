use druid::widget::Slider;
use druid::widget::{CrossAxisAlignment, Flex, Label, TextBox};
use druid::LensExt;
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

pub fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("lens-demo-window-title").with_placeholder("Lens Demo"));
    let data = A { b: B { c: 0.5 } };

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<A> {
    let then = A::b.then(B::c);
    let slider = Slider::new().lens(then);
    let label = Label::new(|d: &A, _: &Env| format!("{:.2}", d.b.c));
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
    b: B,
}
#[derive(Data, Lens, Clone)]
struct B {
    c: f64,
}

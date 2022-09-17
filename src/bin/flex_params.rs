use druid::{AppLauncher, Color, WidgetExt, WindowDesc};

fn main() {
    use druid::widget::{CrossAxisAlignment, FlexParams, Label};

    let mut row = druid::widget::Flex::<()>::row();
    let child_1 = Label::new("I'm hungryI'm hungryI'm hungry").border(Color::RED, 1.0);
    let child_2 = Label::new("I'm scaredI'm scaredI'm scaredI'm scared").border(Color::RED, 1.0);
    // normally you just use a float:
    // child_1 占有空间比= 80 / (80 + 20) = 80%。
    row.add_flex_child(child_1, 80.0);
    row.add_flex_child(child_2, 20.0);

    let main_window = WindowDesc::new(
        row.fix_height(100.0)
            .fix_width(100.0)
            .border(Color::YELLOW, 1.0)
            .debug_paint_layout(),
    )
    .title("Hello World!")
    .window_size((400.0, 400.0));

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
        .expect("Failed to launch application");
}

pub struct Data;

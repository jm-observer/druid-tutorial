use crate::data::{AppData, DynamicTabData, NumberedTabs, TabConfig};
use druid::widget::{
    Axis, Button, CrossAxisAlignment, Flex, Label, Split, TabInfo, Tabs, TabsEdge, TabsPolicy,
    TabsTransition, TextBox,
};
use druid::{Data, Env, Lens, Widget, WidgetExt};

pub fn build_tab_widget(tab_config: &TabConfig) -> impl Widget<AppData> {
    let dyn_tabs = Tabs::for_policy(NumberedTabs)
        .with_axis(tab_config.axis)
        .with_edge(tab_config.edge)
        .with_transition(tab_config.transition)
        .lens(AppData::advanced);

    let control_dynamic = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Control dynamic tabs"))
        .with_child(Button::new("Add a tab").on_click(|_c, d: &mut DynamicTabData, _e| d.add_tab()))
        .with_child(Label::new(|adv: &DynamicTabData, _e: &Env| {
            format!("Highest tab number is {}", adv.highest_tab)
        }))
        .with_spacer(20.)
        .lens(AppData::advanced);

    let first_static_tab = Flex::row()
        .with_child(Label::new("Rename tab:"))
        .with_child(TextBox::new().lens(AppData::first_tab_name));

    let main_tabs = Tabs::new()
        .with_axis(tab_config.axis)
        .with_edge(tab_config.edge)
        .with_transition(tab_config.transition)
        .with_tab(
            |app_state: &AppData, _: &Env| app_state.first_tab_name.to_string(),
            first_static_tab,
        )
        .with_tab("Dynamic", control_dynamic)
        .with_tab("Page 3", Label::new("Page 3 content"))
        .with_tab("Page 4", Label::new("Page 4 content"))
        .with_tab("Page 5", Label::new("Page 5 content"))
        .with_tab("Page 6", Label::new("Page 6 content"))
        .with_tab_index(1);

    Split::rows(main_tabs, dyn_tabs).draggable(true)
}

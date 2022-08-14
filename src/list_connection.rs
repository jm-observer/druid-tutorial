use crate::data::AppData;
use druid::im::{vector, Vector};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll};
use druid::LensExt;
use druid::{lens, Color, UnitPoint, WidgetExt};

pub fn list_connection() -> Flex<AppData> {
    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    // Build a simple list
    // lists.add_flex_child(
    //     Scroll::new(List::new(|| {
    //         Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
    //             .align_vertical(UnitPoint::LEFT)
    //             .padding(10.0)
    //             .expand()
    //             .height(50.0)
    //             .background(Color::rgb(0.5, 0.5, 0.5))
    //     }))
    //     .vertical()
    //     .lens(AppData::left),
    //     1.0,
    // );

    // Build a list with shared data
    lists.add_flex_child(
        Scroll::new(
            List::new(|| {
                Flex::row()
                    .with_child(
                        Label::new(|(_, item): &(Vector<u32>, u32), _env: &_| {
                            format!("List item #{}", item)
                        })
                        .align_vertical(UnitPoint::LEFT),
                    )
                    .with_flex_spacer(1.0)
                    .with_child(
                        Button::new("Delete")
                            .on_click(|_ctx, (shared, item): &mut (Vector<u32>, u32), _env| {
                                // We have access to both child's data and shared data.
                                // Remove element from right list.
                                shared.retain(|v| v != item);
                            })
                            .fix_size(80.0, 20.0)
                            .align_vertical(UnitPoint::CENTER),
                    )
                    .padding(10.0)
                    .background(Color::rgb(0.5, 0.0, 0.5))
                    .fix_height(50.0)
            })
            .with_spacing(10.),
        )
        .vertical()
        .lens(lens::Identity.map(
            // Expose shared data with children data
            |d: &AppData| (d.right.clone(), d.right.clone()),
            |d: &mut AppData, x: (Vector<u32>, Vector<u32>)| {
                // If shared data was changed reflect the changes in our AppData
                d.right = x.0
            },
        )),
        1.0,
    );
    lists
}

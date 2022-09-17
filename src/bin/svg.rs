#![feature(type_name_of_val)]
// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This example shows how to draw an SVG.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::widget::Label;
use druid::{
    theme,
    widget::{FillStrat, Flex, Svg, SvgData, WidgetExt},
    AppLauncher, LocalizedString, Widget, WindowDesc,
};
use rust_embed::RustEmbed;
use std::any::type_name_of_val;
use std::borrow::{Borrow, Cow};

#[derive(RustEmbed)]
#[folder = "icons"]
struct Asset;

pub fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("svg-demo-window-title").with_placeholder("Rawr!"));
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<u32> {
    let tiger_svg = match include_str!("../../icons/arrow-down.svg").parse::<SvgData>() {
        Ok(svg) => svg,
        Err(err) => {
            println!("{}", err);
            println!("Using an empty SVG instead.");
            SvgData::default()
        }
    };
    // let tmp: Cow<'_, str> = Cow::Owned("abc".to_string());
    // let tmp: Cow<'_, [u8]> = Cow::Owned(Vec::new());
    let mut col = Flex::column();
    for item in Asset::iter() {
        if let Some(file) = Asset::get(item.as_ref()) {
            println!(
                "{} {}",
                type_name_of_val(&file),
                type_name_of_val(&file.data)
            );
            let content = String::from_utf8_lossy(file.data.as_ref());
            let svg = content.as_ref().parse::<SvgData>().unwrap();
            let mut row = Flex::row();
            row.add_child(Label::new(item.as_ref()).fix_width(180.0));
            row.add_child(Svg::new(svg).fix_size(20.0, 20.0).debug_paint_layout());
            col.add_child(row);
        }
    }

    // col.add_flex_child(Svg::new(tiger_svg.clone()).fix_width(60.0).center(), 1.0);
    // col.add_flex_child(Svg::new(tiger_svg.clone()).fill_mode(FillStrat::Fill), 1.0);
    // col.add_flex_child(Svg::new(tiger_svg), 1.0);
    col.scroll().background(theme::BACKGROUND_LIGHT)
}

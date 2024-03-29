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

//! This is a very small example of how to setup a druid application.
//! It does the almost bare minimum while still being useful.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::LensExt;
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Hello World!")
        .window_size((400.0, 400.0));

    // create the initial app state
    let count = Count.fetch_add(1, Ordering::Release);
    let initial_state: HelloState = HelloState {
        count,
        other: "other".to_string(),
        name: WrapStr {
            count,
            name: WrapString {
                name: "hello".to_string(),
            },
        },
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label: Label<HelloState> = Label::new(|data: &HelloState, _env: &Env| {
        // println!("{:?}", data);
        if data.name.name.name.is_empty() {
            "Hello anybody!?".to_string()
        } else {
            format!("Hello {:?}!", data.name.name.name)
        }
    })
    .with_text_size(32.0);

    let label1 = Label::new(|data: &WrapStr, _env: &Env| {
        println!("{:?}", data);
        if data.name.name.is_empty() {
            "Hello anybody!?".to_string()
        } else {
            format!("Hello {:?}!", data.name.name)
        }
    })
    .with_text_size(32.0)
    .lens(HelloState::name);
    let label2 = Label::new(|data: &WrapString, _env: &Env| {
        println!("{:?}", data);
        if data.name.is_empty() {
            "Hello anybody!?".to_string()
        } else {
            format!("Hello {:?}!", data.name)
        }
    })
    .with_text_size(32.0)
    .lens(HelloState::name.then(WrapStr::name));
    let label3 = Label::new(|data: &String, _env: &Env| {
        println!("{:?}", data);
        if data.is_empty() {
            "Hello anybody!?".to_string()
        } else {
            format!("Hello {:?}!", data)
        }
    })
    .with_text_size(32.0)
    .lens(HelloState::other);

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name.then(WrapStr::name).then(WrapString::name));

    // arrange the two widgets vertically, with some padding
    Flex::column()
        // .with_child(label)
        // .with_child(label1)
        .with_child(label2)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .align_vertical(UnitPoint::CENTER)
}

static Count: AtomicUsize = AtomicUsize::new(0);

#[derive(Lens, Debug)]
struct HelloState {
    count: usize,
    name: WrapStr,
    other: String,
}

impl ::druid::Data for HelloState {
    fn same(&self, other: &Self) -> bool {
        // println!("HelloState {} other: {}", self.count, other.count);
        druid::Data::same(&self.name.name, &other.name.name)
            && druid::Data::same(&self.other, &other.other)
    }
}

impl Clone for HelloState {
    fn clone(&self) -> Self {
        println!("HelloState.clone");
        let count = Count.fetch_add(1, Ordering::Release);
        Self {
            count,
            other: self.other.clone(),
            name: WrapStr {
                count: count,
                name: self.name.name.clone(),
            },
        }
    }
}
#[derive(Debug, Lens)]
pub struct WrapStr {
    count: usize,
    name: WrapString,
}

impl ::druid::Data for WrapStr {
    fn same(&self, other: &Self) -> bool {
        println!("WrapStr {} other: {}", self.count, other.count);
        let res = self.name.name == other.name.name;
        // let res = druid::Data::same(&self.name, &other.name);
        res
    }
}

impl Deref for WrapStr {
    type Target = WrapString;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl Clone for WrapStr {
    fn clone(&self) -> Self {
        println!("WrapStr.clone");
        Self {
            count: self.count,
            name: self.name.clone(),
        }
    }
}

#[derive(Debug, Lens, Clone)]
pub struct WrapString {
    name: String,
}

impl ::druid::Data for WrapString {
    fn same(&self, other: &Self) -> bool {
        // println!("WrapString same");
        let res = druid::Data::same(&self.name, &other.name);
        res
    }
}

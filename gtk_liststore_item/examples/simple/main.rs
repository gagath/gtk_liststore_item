// SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use gio::prelude::*;
use gtk::prelude::*;
use gtk_liststore_item::ListStoreItem;

use gladis::Gladis;

#[derive(Gladis, Clone)]
struct Ui {
    window: gtk::Window,
    list_store: gtk::ListStore,
    tree_view: gtk::TreeView,
}

#[derive(ListStoreItem)]
struct Item {
    name: String,
    value: u32,
    progress: u32,
    is_cool: bool,
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("simple.ui");
    let mut ui = Ui::from_string(glade_src).unwrap();

    for i in 0..10 {
        let item = Item {
            name: format!("foobar{}", i),
            value: rand::random(),
            progress: rand::random::<u32>() % 100,
            is_cool: rand::random(),
        };
        item.insert_into_liststore(&mut ui.list_store);
    }

    ui.window.set_application(Some(app));
    ui.window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.MicroJoe.gtk_liststore_item.examples.simple"),
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}

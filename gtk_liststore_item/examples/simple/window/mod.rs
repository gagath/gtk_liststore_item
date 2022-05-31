// SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod imp;

use glib::subclass::prelude::*;
use gtk::{gio, glib};

use gtk_liststore_item::ListStoreItem;

use crate::item::Item;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable;
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        // Create new window
        glib::Object::new(&[("application", app)]).expect("Failed to create Window")
    }

    pub fn init_liststore(&self) {
        let self_ = imp::Window::from_instance(self);

        for i in 0..10 {
            let item = Item {
                name: format!("foobar{}", i),
                value: rand::random(),
                progress: rand::random::<u32>() % 100,
                is_cool: rand::random(),
            };
            item.insert_into_liststore(&mut self_.list_store.get());
        }

    }
}
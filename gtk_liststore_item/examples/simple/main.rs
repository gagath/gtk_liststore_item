// SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use gtk::prelude::*;

mod item;
mod window;


fn build_ui(app: &gtk::Application) {
    let window = window::Window::new(app);
    window.present();
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

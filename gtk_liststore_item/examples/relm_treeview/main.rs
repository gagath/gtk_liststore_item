// SPDX-FileCopyrightText: 2018 Boucher, Antoni <bouanto@zoho.com>
// SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: MIT

use std::fs;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use gtk::prelude::*;

// use glib::StaticType;
use gtk::Orientation::Vertical;
use gtk::{
    prelude::CellLayoutExt, prelude::ContainerExt, prelude::GtkListStoreExtManual,
    prelude::GtkWindowExt, prelude::TreeModelExt, prelude::TreeSelectionExt, prelude::TreeViewExt,
    prelude::WidgetExt, Inhibit, TreeView, Window, WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

use gtk_liststore_item::ListStoreItem;

// These two constants stand for the columns of the listmodel and the listview
//const VALUE_COL: i32 = 0;
//const IS_DIR_COL: i32 = 1;

struct Directory {
    current_dir: PathBuf,
}

#[derive(Msg)]
enum Msg {
    ItemSelect,
    Quit,
}

struct Win {
    tree_view: TreeView,
    model: Directory,
    window: Window,
}

#[derive(ListStoreItem)]
struct DirModel {
    name: String,
    is_dir: bool,
}

impl Update for Win {
    type Model = Directory;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Directory {
        let working_directory = fs::canonicalize(".").expect("Failed to open directory");
        Directory {
            current_dir: working_directory,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::ItemSelect => {
                let selection = self.tree_view.selection();
                if let Some((list_model, iter)) = selection.selected() {
                    //let is_dir: bool = list_model
                    //    .value(&iter, IS_DIR_COL)
                    //    .get::<bool>()
                    //    .ok()
                    //    .expect("get_value.get<bool> failed");
                    let dir = DirModel::from_liststore_iter(&list_model, &iter).unwrap();
                    let is_dir = dir.is_dir;
                    let dir_name = dir.name;

                    if is_dir {
                        //let dir_name = list_model
                        //    .value(&iter, VALUE_COL)
                        //    .get::<String>()
                        //    .ok()
                        //    .expect("get_value.get<String> failed");

                        println!("{:?} selected", dir_name);
                        let new_dir = if dir_name == ".." {
                            // Go up parent directory, if it exists
                            self.model
                                .current_dir
                                .parent()
                                .unwrap_or(&self.model.current_dir)
                                .to_owned()
                        } else {
                            self.model.current_dir.join(dir_name)
                        };
                        self.model.current_dir = new_dir;
                        let new_model = create_and_fill_model(&self.model.current_dir)
                            .expect("create_and_fill_model failed");

                        self.tree_view.set_model(Some(&new_model));
                    }
                }
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = gtk::Window::new(WindowType::Toplevel);
        let vbox = gtk::Box::new(Vertical, 0);
        let tree_view = gtk::TreeView::new();
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();

        window.set_title("TreeView example file browser");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);

        column.pack_start(&cell, true);
        // Assiciate view's column with model's id column
        column.add_attribute(&cell, "text", 0);
        tree_view.append_column(&column);

        let store_model =
            create_and_fill_model(&model.current_dir).expect("create_and_fill_model failed");
        tree_view.set_model(Some(&store_model));

        vbox.add(&tree_view);
        window.add(&vbox);

        window.show_all();

        connect!(relm, tree_view, connect_cursor_changed(_), Msg::ItemSelect);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            tree_view,
            model,
            window,
        }
    }
}

fn create_and_fill_model(dir_str: &Path) -> io::Result<gtk::ListStore> {
    // Single row model
    //let model = gtk::ListStore::new(&[String::static_type(), bool::static_type()]);
    let mut model = DirModel::new_liststore();

    // Add the parent directory
    //model.insert_with_values(None, &[(VALUE_COL as u32, &".."), (IS_DIR_COL as u32, &true)]);
    let parent = DirModel {
        name: "..".into(),
        is_dir: true,
    };
    parent.insert_into_liststore(&mut model);

    let entry_iter = fs::read_dir(dir_str)?.filter_map(|x| x.ok());
    for entry in entry_iter {
        if let Ok(metadata) = entry.metadata() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let (final_name, is_dir) = if metadata.is_dir() {
                    (format!("{}{}", file_name, MAIN_SEPARATOR), true)
                } else {
                    (file_name, false)
                };
                //model.insert_with_values(None,
                //                        &[(VALUE_COL as u32, &final_name), (IS_DIR_COL as u32, &is_dir)]);
                let name = final_name;
                let value = DirModel { name, is_dir };
                value.insert_into_liststore(&mut model);
            }
        }
    }
    Ok(model)
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

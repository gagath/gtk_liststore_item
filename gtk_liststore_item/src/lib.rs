use gtk::prelude::*;

pub trait ListStoreItem {
    fn from_list_store(list_store: gtk::ListStore, tp: &gtk::TreePath) -> Option<Self> where Self: std::marker::Sized;
    fn insert_to_list_store(&self, list_store: gtk::ListStore) -> gtk::TreeIter;
}
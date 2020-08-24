# gtk_liststore_item

[![Latest version](https://img.shields.io/crates/v/gtk_liststore_item.svg)](https://crates.io/crates/gtk_liststore_item)
[![Documentation](https://docs.rs/gtk_liststore_item/badge.svg)](https://docs.rs/gtk_liststore_item)
[![License](https://img.shields.io/crates/l/gtk_liststore_item.svg)](https://crates.io/crates/gtk_liststore_item)

Automatic `gtk::ListStore` struct derive for Rust.

## Usage

In order to use this crate, you have to add the following dependencies into
your project's `Cargo.toml` file:

```toml
[dependencies]
gtk_liststore_item = "0.1.2"
```

## Example

After the crate is installed, you can enjoy the `ListStoreItem` derive!

```rust
#[derive(ListStoreItem)]
struct Item {
    name: String,
    value: u32,
}

fn main() {
    gtk::init().unwrap();

    // Depends if you are using Glade or code for your UI
    let list_store = ...;

    let item = Item { name: "foobar".into(), value: 42 };
    item.insert_into_liststore(list_store);
}
```

Without this crate, you would have to manually serialize each of the entries in
your struct with their type and position:

```rust
fn get_item(liststore: gtk::ListStore, iter: &gtk::TreeIter) {
    Some(Item {
        name: list_store.get_value(&iter, 0).get::<String>().ok()??,
        value: list_store.get_value(&iter, 1).get::<u32>().ok()??,
    })
}

fn insert_item(item: &Item, list_store: gtk::ListStore) -> gtk::TreeIter {
    list_store.insert_with_values(
        None,
        &[0, 1],
        &[&self.name, &self.value])
}
```

This can become pretty boring, hence this crate to ease the process.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

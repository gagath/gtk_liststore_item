//! A Rust crate to easily import Glade-generated UI files into Rust code (proc
//! macros).

use proc_macro::TokenStream;
use quote::quote;

use syn::{Data, DataStruct, DeriveInput, Fields};

fn impl_liststore_item(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);
    let field_number = 0..fields.len() as i32;
    let from = quote! {
        fn from_liststore_iter<S>(list_store: &S, iter: &gtk::TreeIter) -> Option<Self>
            where S: TreeModelExt
        {
            Some(#name {
                #(
                    #field_name: list_store.value(iter, #field_number).get::<#field_type>().ok()?
                ),*
            })
        }
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_pos = 0..fields.len();

    let insert = quote! {
        fn insert_into_liststore<S>(&self, list_store: &mut S) -> gtk::TreeIter
            where S: GtkListStoreExtManual
        {
            list_store.insert_with_values(
                None,
                &[
                    #(
                        (
                            #field_pos as u32,
                            &self.#field_name
                        )
                    ),*
                ],
            )
        }
    };

    let field_type = fields.iter().map(|field| &field.ty);
    let new_liststore = quote! {
        fn new_liststore() -> gtk::ListStore {
            gtk::ListStore::new(
                &[#(
                    #field_type::static_type()),*
                ]
            )
        }
    };

    let gen = quote! {
        impl gtk_liststore_item::ListStoreItem for #name {
            #from
            #insert
            #new_liststore
        }
    };
    gen.into()
}

#[proc_macro_derive(ListStoreItem)]
pub fn derive_liststore_item(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_liststore_item(&ast)
}

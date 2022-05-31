// SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use gtk_liststore_item::ListStoreItem;
use gtk::traits::TreeModelExt;
use gtk::prelude::*;

#[derive(ListStoreItem)]
pub struct Item {
    pub name: String,
    pub value: u32,
    pub progress: u32,
    pub is_cool: bool,
}
// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use cursive::{
    utils::markup::StyledString,
    views::{EditView, SelectView, TextView},
    Cursive,
};
use std::sync::Arc;

pub trait GbHwDbCursiveExt {
    fn try_get_edit_view_value(&mut self, id: &str) -> Option<String>;
    fn get_edit_view_value(&mut self, id: &str) -> String;
    fn get_select_view_selection<T>(&mut self, id: &str) -> Option<T>
    where
        T: Clone + Send + Sync + 'static;
    fn set_text_view_content<S: Into<StyledString>>(&mut self, id: &str, content: S);
}

impl GbHwDbCursiveExt for Cursive {
    fn try_get_edit_view_value(&mut self, id: &str) -> Option<String> {
        self.call_on_name(id, |view: &mut EditView| String::clone(&view.get_content()))
    }
    fn get_edit_view_value(&mut self, id: &str) -> String {
        self.try_get_edit_view_value(id)
            .unwrap_or_else(|| panic!("No EditView with id {:?}", id))
    }
    fn get_select_view_selection<T>(&mut self, id: &str) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        self.call_on_name(id, |view: &mut SelectView<T>| {
            view.selection().map(Arc::unwrap_or_clone)
        })
        .unwrap_or_else(|| panic!("No SelectView with id {:?}", id))
    }
    fn set_text_view_content<S: Into<StyledString>>(&mut self, id: &str, content: S) {
        self.call_on_name(id, |view: &mut TextView| view.set_content(content))
            .unwrap_or_else(|| panic!("No TextView with id {:?}", id))
    }
}

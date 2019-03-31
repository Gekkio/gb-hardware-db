use cursive::utils::markup::StyledString;
use cursive::views::{EditView, SelectView, TextView};
use cursive::Cursive;

pub trait CursiveExt {
    fn get_edit_view_value(&mut self, id: &str) -> String;
    fn get_select_view_selection<T>(&mut self, id: &str) -> Option<T>
    where
        T: Clone + 'static;
    fn set_text_view_content<S: Into<StyledString>>(&mut self, id: &str, content: S);
}

impl CursiveExt for Cursive {
    fn get_edit_view_value(&mut self, id: &str) -> String {
        self.call_on_id(id, |view: &mut EditView| String::clone(&view.get_content()))
            .unwrap_or_else(|| panic!("No EditView with id {:?}", id))
    }
    fn get_select_view_selection<T>(&mut self, id: &str) -> Option<T>
    where
        T: Clone + 'static,
    {
        self.call_on_id(id, |view: &mut SelectView<T>| {
            view.selection().as_ref().map(|s| T::clone(s))
        })
        .unwrap_or_else(|| panic!("No SelectView with id {:?}", id))
    }
    fn set_text_view_content<S: Into<StyledString>>(&mut self, id: &str, content: S) {
        self.call_on_id(id, |view: &mut TextView| view.set_content(content))
            .unwrap_or_else(|| panic!("No TextView with id {:?}", id))
    }
}

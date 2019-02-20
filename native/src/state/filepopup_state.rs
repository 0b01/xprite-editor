use crate::prelude::*;

pub struct FilePopupState {
    pub show_file_popup: bool,
    pub open_file_name: ImString,
    pub show_file_is_save: bool,
}

impl Default for FilePopupState {
    fn default() -> Self {
        Self {
            show_file_popup: false,
            show_file_is_save: true,
            open_file_name: ImString::new("./sample_files/1.ase"),
        }
    }
}

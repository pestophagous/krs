use std::path::Path;

use crate::krs_base::set;

pub struct ExtractText {
    files: Vec<String>,
}

impl ExtractText {
    // Another associated function, taking two arguments:
    pub fn new(inputfiles: Vec<String>) -> ExtractText {
        for f in &inputfiles {
            let ok = Path::new(&f).is_file();
            if !ok {
                panic!("\nPANIC REASON: file does not exist: {f}\n\n");
            }
        }

        ExtractText { files: inputfiles }
    }

    pub fn parse(&self) -> set::Set {
        let input_set = set::Set::new();
        input_set
    }
}

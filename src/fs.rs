use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

pub fn get_files<T: AsRef<Path>>(target_path: T, ext_name: &String) -> Vec<PathBuf> {
    let mut r = vec![];
    for entry in WalkDir::new(target_path) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() == true {
            if entry.path().extension().unwrap().to_str().unwrap() == ext_name {
                // println!("{}", entry.path().display());

                r.push(entry.into_path());
            }
        }
    }

    return r;
}

#[test] // cargo test --lib -- fs::test_foo --nocapture
fn test_foo() {
    let a = get_files("../typ-p/proto/", &"proto".into());

    println!("{:?}", a);
}

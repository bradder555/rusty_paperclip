// source https://stackoverflow.com/questions/57535794/how-do-i-include-a-folder-in-the-building-process

use std::{
    env, fs,
    path::{Path, PathBuf},
};


const COPY_DIRS: &'static [&'static str] = &[

];

const COPY_FILES: &'static [&'static str] = &[
    "./assets/config.yaml"
];

fn copy_file(from: &Path, to: &Path) {
    let path_str = from.as_os_str().to_str().unwrap_or_default();
    fs::copy(from, to).expect(&format!("couldn't move file {}", path_str));
}

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());
        
        if path.is_file() {
            copy_file(path.as_path(), to.as_path())
                

        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        } else { /* Skip other content */
        }
    }
}

fn main() {
    // Request the output directory
    let out = env::var("OUT_DIR").unwrap();
    let out = PathBuf::from(&out);

    // it's a bit hacky, but this will put the file in the actual output dir
    // will do for now!
    let out = out.join("../../../");
    println!("cargo:warning={}", out.as_path().as_os_str().to_str().unwrap());

    /*
    for (k,v) in env::vars(){
        println!("cargo:warning={}: {}", k,v );
    }
    */

    for dir in COPY_DIRS {

        // Create the out directory
        fs::create_dir(&out).unwrap();

        // Copy the directory
        copy_dir(dir , &out);

    }

    for file in COPY_FILES {
        let from = PathBuf::from(*file);
        let filename = from.file_name().expect("no filename?");
        let to = out.join(filename);
        copy_file(&from, &to);
    }
}
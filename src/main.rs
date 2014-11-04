extern crate inotify;

use inotify::wrapper::INotify;

use std::vec::Vec;
use std::os;

use package::Package;

pub mod package;

fn main() {
    let repo_name = "siosm-aur";
    let db_filename = "siosm-aur.db";
    let package_suffix_length = ".pkg.tar.xz".len();
    let signature_suffix_length = ".sig".len();

    let package_name = "lnav-0.5.1-1-x86_64.pkg.tar.xz";

    let mut files_ready: Vec<String>;

    let mut handler = INotify::init().unwrap();
    handler.add_watch(&os::getcwd(), inotify::ffi::IN_CLOSE_WRITE).unwrap();

    loop {
        let event = handler.event().unwrap();
        // println!("{} {}", event.name, event.mask);
        spawn(proc () {
            let filename = event.name;
            println!("Handling file: {}", filename);
            // if filename.as_slice() == db_filename
            //     || filename.as_slice() == db_filename + ".tar.xz" {
            //     println!("Ignoring file: {}", filename);
            //     return;
            // }
            // let package_name =
            //     if filename.slice_from_or_fail(filename.len() - ) {
            //     }

            // for file in files_ready {
            //     if file == filename {
            //     }
            // }
        });
    }
}

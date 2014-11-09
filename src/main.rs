extern crate inotify;

use inotify::wrapper::INotify;

use std::vec::Vec;
use std::os;

use package::Package;

pub mod package;

fn main() {
    let repo_name = "siosm-aur";
    // let db_filename = "siosm-aur.db";

    // let mut packages_ready = Vec::new();
    // let mut signatures_ready = Vec::new();

    println!("Looking for packages in repo {}", repo_name);

    let mut handler = INotify::init().unwrap();
    handler.add_watch(&os::getcwd(), inotify::ffi::IN_CLOSE_WRITE).unwrap();

    loop {
        let event = handler.event().unwrap();
        // println!("{} {}", event.name, event.mask);
        spawn(proc () {
            let filename = event.name;
            println!("Handling file: {}", filename);
            // let package = match from_str::<Package>(filename.as_slice()) {
            //     None    => return,
            //     Some(p) => {
            //         if p.is_signed() {
            //             signatures_ready.push(p);
            //         } else {
            //             packages_ready.push(p);
            //         }
            //     }
            // };
        });
    }
}

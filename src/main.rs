#![feature(phase)]

#[phase(plugin, link)] extern crate log;
extern crate collections;
extern crate inotify;

use inotify::wrapper::INotify;

// use collections::str::FromStr;
// use collections::string::String;

use std::vec::Vec;
use std::os;

use package::Package;

mod package;

enum Action {
    ADD,
    REMOVE
}

fn main() {
    // let repo_name = "siosm-aur";
    // let db_filename = "siosm-aur.db";

    let mut packages = Vec::new();

    let repo_name = "siosm-aur";
    // let repo_name = match os::getcwd() {
    //     Err(e) => panic!("Current directory is invalid: {}", e),
    //     Ok(p)  => match p.filename() {
    //         None    => panic!("Current directory is invalid: {}", p),
    //         Some(b) => match String::from_utf8(b) {
    //             Err(e) => panic!("FIXME: find the cases when this is triggered: {}", e),
    //             Ok(s)  => s,
    //         }
    //     }
    // };
    info!("Looking for packages in repo: {}", repo_name);

    let mut handler = INotify::init().unwrap();
    let mask = inotify::ffi::IN_CLOSE_WRITE | inotify::ffi::IN_DELETE
               | inotify::ffi::IN_MOVED_FROM | inotify::ffi::IN_MOVED_TO;
    handler.add_watch(&os::getcwd(), mask).unwrap();

    loop {
        let event = handler.event().unwrap();
        let filename = event.name;
        let action = match event.mask {
            inotify::ffi::IN_CLOSE_WRITE | inotify::ffi::IN_MOVED_TO => Action::ADD,
            inotify::ffi::IN_DELETE | inotify::ffi::IN_MOVED_FROM => Action::REMOVE,
            _ => panic!("This event is not handled!")
        };
        info!("Handling file: {}", filename);

        // For now, the daemon is waiting for a magic file named 'DONE' before
        // operating on the database.
        if filename.as_slice() != "DONE" {
            match from_str::<Package>(filename.as_slice()) {
                None    => info!("Ignoring file: {}", filename),
                Some(p) => {
                    match action {
                        Action::ADD    => {
                            info!("Adding file: {}", p);
                            packages.push(p)
                        },
                        Action::REMOVE => {
                            info!("Removing file: {}", p);
                            packages.push(p)
                        }
                    }
                }
            };
        } else {
            println!("Found DONE file");
            // Update repo database with the new packages
            println!("Looking for packages to add to the database...");
            let (ready, remaining) = packages.partitioned(|p| p.is_binary() && p.is_signed());
            println!("The following packages will be added/updated:");
            println!("{}", ready);
            println!("The following packages will be NOT added/updated:");
            println!("{}", remaining);
            packages = remaining;
        }
    }
}

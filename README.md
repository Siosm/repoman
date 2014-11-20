# repoman, an Arch Linux repository manager

This is a work in progress to implement a repository manager for Arch Linux
packages in Rust.

The goal is to auto-manage an Arch Linux package repository by simply allowing
users to upload package to a specific folder and let them tell the daemon when
they are done. At this point the daemon will look at what has been uploaded,
will check the OpenPGP signatures and will add / remove packages from the
specified repository database.

This should enable painless repository update from untrusted users, which means
that one could host an Arch Linux repository hosting service with
Rust-safety-based confidence that nothing bad will happen.

Package upload may happen from any classic mean (ssh, ftp, rsync) but will
currently not work with NFS.

## Current features

* Detect new packages moved / created in a specific folder

## TODO

### Expected features

- Add packages to the database when told to do so
- Parse command line arguments or read a configuration file for options

* Check package signature validity before adding them to the database
* Make the daemon stateless
* Manage multiple version of each package and store them in a backup folder
* Manage multiple repositories and databases with a single daemon

### Later

* Remove the need for the `DONE` file (if possible and if it makes sense); This
  would also allow NFS backed repo support (see Notice)
* Delta package support
* Rewrite repo-add and repo-remove in Rust
  * Rewrite basic database functionality
  * Add signature support
  * Integrate everything in a single daemon and provide command line tools to
    query it over a socket / dbus / kdbus for example

## How to build

This is developed against Rust nightly builds.

```
$ cargo build
$ cargo test
```

## How to run

You may use the example systemd unit file `repoman-example.service`.

## Packages

None for the moment as Rust is still unstable.

## Notice

This daemon is relying on the Linux inotify interface to function properly.
Thus any limitations and caveats from this interface also apply to this daemon
(see [inotify(7)](http://man7.org/linux/man-pages/man7/inotify.7.html)).

## License

Licensed under the MIT license, see LICENSE.

use std::from_str::FromStr;
use std::fmt::{Formatter, FormatError, Show};
use std::option::Option;
use std::cmp::Eq;

#[deriving(Clone, PartialEq)]
pub enum Arch {
    i686,
    x86_64,
    any
}

impl Show for Arch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &i686   => write!(f, "i686"),
            &x86_64 => write!(f, "x86_64"),
            &any    => write!(f, "any")
        }
    }
}

#[deriving(Clone, PartialEq)]
pub struct Package {
    pkgname: String,
    pkgver: String,
    pkgrel: uint,
    epoch: uint,
    arch: Arch
}

impl Package {
    pub fn new(pkgname: &str, pkgver: &str, pkgrel: uint, epoch: uint,
               arch: Arch) -> Package {
        Package {
            pkgname: String::from_str(pkgname),
            pkgver: String::from_str(pkgver),
            pkgrel: pkgrel,
            epoch: epoch,
            arch: arch,
        }
    }
}

impl FromStr for Package {
    fn from_str(s: &str) -> Option<Package> {
        for i in s.graphemes(true).rev() {
            // if 
        }
    }
}

impl Show for Package {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        if self.epoch == 0 {
            write!(f, "{}-{}-{}-{}.pkg.tar.xz", self.pkgname, self.pkgver,
                   self.pkgrel, self.arch)
        } else {
            write!(f, "{}-{}:{}-{}-{}.pkg.tar.xz", self.pkgname, self.epoch,
                   self.pkgver, self.pkgrel, self.arch)
        }
    }
}

#[test]
fn package_from_string_parsing() {
    let package1: Package = from_str("lnav-0.5.1-1-x86_64.pkg.tar.xz");
    let package2 = Package::new("lnav", "0.5.1", 1, 0, x86_64);
    assert!(package1, package2);
}

#[test]
fn package_to_string() {
    let filename = "lnav-0.5.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("lnav", "0.5.1", 1, 0, x86_64);
    assert!(filename, format!("{}", package));
}

#[test]
fn package_from_and_to_string() {
    let filename = "lnav-0.5.1-1-x86_64.pkg.tar.xz";
    let package: Package = from_str("lnav", "0.5.1", 1, 0, x86_64);
    assert!(filename, format!("{}", package));
}


#[test]
fn package_with_epoch_from_string_parsing() {
    let package1: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz");
    let package2 = Package::new("docker", "1.3.1", 1, 1, x86_64);
    assert!(package1, package2);
}

#[test]
fn package_with_epoch_to_string() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("docker", "1.3.1", 1, 1, x86_64);
    assert!(filename, format!("{}", package));
}

#[test]
fn package_with_epoch_from_and_to_string() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz";
    let package: Package = from_str("docker", "1.3.1", 1, 1, x86_64);
    assert!(filename, format!("{}", package));
}

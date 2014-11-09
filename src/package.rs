use std::from_str::FromStr;
use std::fmt::{Formatter, FormatError, Show};
use std::option::Option;

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
    pkgname:   String,
    pkgver:    String,
    pkgrel:    uint,
    epoch:     uint,
    arch:      Arch,
    signature: bool
}

impl Package {
    pub fn new(pkgname: &str, pkgver: &str, pkgrel: uint, epoch: uint,
               arch: Arch, signature: bool) -> Package {
        Package {
            pkgname:   String::from_str(pkgname),
            pkgver:    String::from_str(pkgver),
            pkgrel:    pkgrel,
            epoch:     epoch,
            arch:      arch,
            signature: signature
        }
    }
}

impl FromStr for Package {
    fn from_str(s: &str) -> Option<Package> {
        let mut package = Package::new("", "", 1, 0, x86_64, false);
        let mut tmp = "";

        println!("Looking at {}", s);
        // Looking for signature suffix
        if s.ends_with(".sig") {
            package.signature = true;
            tmp = s.slice_to(s.len() - ".sig".len());
            println!("Found signature suffix");
        } else {
            println!("No signature suffix found");
        }

        println!("Looking at {}", if tmp == "" { s } else { tmp });
        // Looking for package prefix
        if tmp.ends_with(".pkg.tar.xz") {
            tmp = tmp.slice_to(tmp.len() - ".pkg.tar.xz".len());
            println!("Found package suffix");
        } else {
            println!("No package prefix found!");
            return None
        }

        println!("Looking at {}", tmp);
        // Looking for architecture
        let mut pos = match tmp.rfind('-') {
            None    => return None,
            Some(x) => x
        };
        println!("Found '-' at pos {} in {}", pos, tmp);
        package.arch = match tmp.slice_from(pos + 1) {
            "i686"   => i686,
            "x86_64" => x86_64,
            "any"    => any,
            _        => return None
        };
        tmp = tmp.slice_to(pos);
        println!("Package arch is {}", package.arch);

        println!("Looking at {}", tmp);
        // Looking for pkgrel
        pos = match tmp.rfind('-') {
            None    => return None,
            Some(x) => x
        };
        println!("Found '-' at pos {} in {}", pos, tmp);
        let rel: Option<uint> = from_str(tmp.slice_from(pos + 1));
        match rel {
            None    => return None,
            Some(x) => {
                package.pkgrel = x;
                tmp = tmp.slice_to(pos);
            }
        }
        println!("Package pkgrel is {}", package.pkgrel);

        println!("Looking at {}", tmp);
        // Looking for epoch+pkgver
        pos = match tmp.rfind('-') {
            None    => return None,
            Some(x) => x
        };
        println!("Found '-' at pos {} in {}", pos, tmp);
        pos = match tmp.slice_from(pos + 1).find(':') {
            None    => pos,
            Some(x) => {
                println!("Found ':' at pos {} in {}", pos + 1 + x, tmp);
                let epoch: Option<uint> = from_str(tmp.slice_chars(pos + 1, pos + 1 + x));
                match epoch {
                    None    => return None,
                    Some(z) => {
                        package.epoch = z;
                        pos + 1 + x
                    }
                }
            }
        };
        println!("Package epoch is {}", package.epoch);
        println!("Looking at {}", tmp);
        package.pkgver = match from_str(tmp.slice_from(pos + 1)) {
            None    => return None,
            Some(s) => s
        };
        tmp = tmp.slice_to(pos);
        println!("Package pkgver is {}", package.pkgver);

        println!("Looking at {}", tmp);
        // Everything else is the pkgname
        package.pkgname = match from_str(tmp) {
            None    => return None,
            Some(s) => s
        };
        println!("Package name is {}", package.pkgname);

        Some(package)
    }
}

impl Show for Package {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let version = if self.epoch == 0 {
            format!("{}", self.pkgver)
        } else {
            format!("{}:{}", self.epoch, self.pkgver)
        };

        let suffix = if self.signature {
            ".sig"
        } else {
            ""
        };

        write!(f, "{}-{}-{}-{}.pkg.tar.xz{}", self.pkgname, version,
               self.pkgrel, self.arch, suffix)
    }
}


// Testing without epoch and signature

#[test]
fn package_from_string_parsing() {
    let package1: Package = from_str("lnav-0.5.1-1-x86_64.pkg.tar.xz").unwrap();
    let package2 = Package::new("lnav", "0.5.1", 1, 0, x86_64, false);
    assert_eq!(package1, package2);
}

#[test]
fn package_to_string() {
    let filename = "lnav-0.5.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("lnav", "0.5.1", 1, 0, x86_64, false);
    assert_eq!(filename, format!("{}", package).as_slice());
}

#[test]
fn package_from_and_to_string() {
    let filename = "lnav-0.5.1-1-x86_64.pkg.tar.xz";
    let package: Package = from_str("lnav-0.5.1-1-x86_64.pkg.tar.xz").unwrap();
    assert_eq!(filename, format!("{}", package).as_slice());
}


// Testing with epoch and no signature

#[test]
fn package_with_epoch_from_string_parsing() {
    let package1: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz").unwrap();
    let package2 = Package::new("docker", "1.3.1", 1, 1, x86_64, false);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_to_string() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("docker", "1.3.1", 1, 1, x86_64, false);
    assert_eq!(filename, format!("{}", package).as_slice());
}

#[test]
fn package_with_epoch_from_and_to_string() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz";
    let package: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz").unwrap();
    assert_eq!(filename, format!("{}", package).as_slice());
}


// Testing with epoch and signature

#[test]
fn package_with_epoch_and_sign_from_string_parsing() {
    let package1: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig").unwrap();
    let package2 = Package::new("docker", "1.3.1", 1, 1, x86_64, true);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_and_sign_to_string() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package = Package::new("docker", "1.3.1", 1, 1, x86_64, true);
    assert_eq!(filename, format!("{}", package).as_slice());
}

#[test]
fn package_with_epoch_and_sign_from_and_to_string() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig").unwrap();
    assert_eq!(filename, format!("{}", package).as_slice());
}

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

    pub fn is_signed(&self) -> bool {
        self.signature
    }
}

impl FromStr for Package {
    fn from_str(s: &str) -> Option<Package> {
        let mut package = Package::new("", "", 1, 0, x86_64, false);
        let mut tmp = s;

        println!("Looking at {}", s);
        // Looking for signature suffix
        if tmp.ends_with(".sig") {
            package.signature = true;
            tmp = tmp.slice_to(s.len() - ".sig".len());
            println!("Found signature suffix");
        } else {
            println!("No signature suffix found!");
        }

        println!("Looking at {}", tmp);
        // Looking for package prefix
        if tmp.ends_with(".pkg.tar.xz") {
            tmp = tmp.slice_to(tmp.len() - ".pkg.tar.xz".len());
            println!("Found package suffix");
        } else {
            println!("No package suffix found!");
            return None
        }

        let vec: Vec<&str> = tmp.rsplitn(3, '-').collect();
        if vec.len() != 4 {
            println!("Invalid package name format: missing at least one element, only {} found!", vec.len());
            return None
        }

        println!("Found the following fields: {}", vec);

        // Looking for architecture
        println!("Looking for arch in {}", vec[0]);
        package.arch = match vec[0] {
            "i686"   => i686,
            "x86_64" => x86_64,
            "any"    => any,
            _        => return None
        };
        println!("Package arch is {}", package.arch);

        // Looking for pkgrel
        println!("Looking for pkgrel in {}", vec[1]);
        package.pkgrel = match from_str::<uint>(vec[1]) {
            None    => return None,
            Some(x) => {
                if x == 0 {
                    println!("Invalid pkgrel, must be > 0!");
                    return None
                } else {
                    x
                }
            }
        };
        println!("Package pkgrel is {}", package.pkgrel);

        // Looking for epoch+pkgver
        println!("Looking for epoch in {}", vec[2]);
        let version: Vec<&str> = vec[2].splitn(2, ':').collect();
        if version.is_empty() {
            println!("Empty version field!");
            return None
        } else if version.len() == 1 {
            package.pkgver = match from_str(vec[2]) {
                None    => return None,
                Some(s) => s
            };
            println!("Package version is {}", package.pkgver);
        } else {
            package.epoch  = match from_str(version[0]) {
                None    => return None,
                Some(x) => x
            };
            println!("Package epoch is {}", package.epoch);
            package.pkgver = match from_str(version[1]) {
                None    => return None,
                Some(s) => s
            };
            println!("Package version is {}", package.pkgver);
        }

        // Everything else is the pkgname
        println!("Looking for name in {}", vec[3]);
        package.pkgname = match from_str(vec[3]) {
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
fn package_from_string() {
    let package1: Package = from_str("lnav-0.5.1-1-x86_64.pkg.tar.xz").unwrap();
    let package2 = Package::new("lnav", "0.5.1", 1, 0, x86_64, false);
    assert_eq!(package1, package2);
}

#[test]
fn package_format() {
    let filename = "lnav-0.5.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("lnav", "0.5.1", 1, 0, x86_64, false);
    assert_eq!(filename, format!("{}", package).as_slice());
}

#[test]
fn package_from_string_and_format() {
    let filename = "lnav-0.5.1-1-x86_64.pkg.tar.xz";
    let package: Package = from_str("lnav-0.5.1-1-x86_64.pkg.tar.xz").unwrap();
    assert_eq!(filename, format!("{}", package).as_slice());
}


// Testing with epoch and no signature

#[test]
fn package_with_epoch_from_string() {
    let package1: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz").unwrap();
    let package2 = Package::new("docker", "1.3.1", 1, 1, x86_64, false);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_format() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("docker", "1.3.1", 1, 1, x86_64, false);
    assert_eq!(filename, format!("{}", package).as_slice());
}

#[test]
fn package_with_epoch_from_string_and_format() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz";
    let package: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz").unwrap();
    assert_eq!(filename, format!("{}", package).as_slice());
}


// Testing with epoch and signature

#[test]
fn package_with_epoch_and_sign_from_string() {
    let package1: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig").unwrap();
    let package2 = Package::new("docker", "1.3.1", 1, 1, x86_64, true);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_and_sign_format() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package = Package::new("docker", "1.3.1", 1, 1, x86_64, true);
    assert_eq!(filename, format!("{}", package).as_slice());
}

#[test]
fn package_with_epoch_and_sign_from_string_and_format() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package: Package = from_str("docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig").unwrap();
    assert_eq!(filename, format!("{}", package).as_slice());
}


// Testing with epoch, signature and dash in the name

#[test]
fn package_with_epoch_dash_and_sign_from_string() {
    let package1: Package = from_str("docker-test-foo-1:1.3.1-1-x86_64.pkg.tar.xz.sig").unwrap();
    let package2 = Package::new("docker-test-foo", "1.3.1", 1, 1, x86_64, true);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_dash_and_sign_format() {
    let filename = "docker-test-foo-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package = Package::new("docker-test-foo", "1.3.1", 1, 1, x86_64, true);
    assert_eq!(filename, format!("{}", package).as_slice());
}

#[test]
fn package_with_epoch_dash_and_sign_from_string_and_format() {
    let filename = "docker-test-foo-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package: Package = from_str("docker-test-foo-1:1.3.1-1-x86_64.pkg.tar.xz.sig").unwrap();
    assert_eq!(filename, format!("{}", package).as_slice());
}


// Testing error paths

#[test]
#[should_fail]
fn package_missing_suffix() {
    from_str::<Package>("docker-1:1.3.1-1-x86_64").unwrap();
}

#[test]
#[should_fail]
fn package_missing_version() {
    from_str::<Package>("docker-1-x86_64.pkg.tar.xz").unwrap();
}

#[test]
#[should_fail]
fn package_invalid_pkgrel() {
    from_str::<Package>("docker-1:1.3.1-0-x86_64.pkg.tar.xz.sig").unwrap();
}

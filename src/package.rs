use collections::str::FromStr;

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
            &Arch::i686   => write!(f, "i686"),
            &Arch::x86_64 => write!(f, "x86_64"),
            &Arch::any    => write!(f, "any")
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
    binary:    bool,
    signature: bool
}

impl Package {
    pub fn new(pkgname: &str, pkgver: &str, pkgrel: uint, epoch: uint,
               arch: Arch, binary: bool, signature: bool) -> Package {
        Package {
            pkgname:   String::from_str(pkgname),
            pkgver:    String::from_str(pkgver),
            pkgrel:    pkgrel,
            epoch:     epoch,
            arch:      arch,
            binary:    binary,
            signature: signature
        }
    }

    pub fn set_binary(&mut self, value: bool) {
        self.binary = value;
    }

    pub fn set_signed(&mut self, value: bool) {
        self.signature = value;
    }

    pub fn is_binary(&self) -> bool {
        self.binary
    }

    pub fn is_signed(&self) -> bool {
        self.signature
    }
}

impl FromStr for Package {
    fn from_str(s: &str) -> Option<Package> {
        let mut package = Package::new("", "", 1, 0, Arch::x86_64, false, false);
        let mut tmp = s;

        debug!("{}: looking for signature suffix in {}", s, s);
        if tmp.ends_with(".sig") {
            package.signature = true;
            tmp = tmp.slice_to(s.len() - ".sig".len());
            debug!("{}: found signature suffix", s);
        } else {
            package.binary = true;
            debug!("{}: no signature suffix found", s);
        }

        debug!("{}: looking for package prefix in {}", s, tmp);
        if tmp.ends_with(".pkg.tar.xz") {
            tmp = tmp.slice_to(tmp.len() - ".pkg.tar.xz".len());
            debug!("{}: found package suffix", s);
        } else {
            debug!("{}: no package suffix found", s);
            return None
        }

        let vec: Vec<&str> = tmp.rsplitn(3, '-').collect();
        if vec.len() != 4 {
            debug!("{}: invalid package name format: missing at least one \
                  element, only {} found", s, vec.len());
            return None
        }
        debug!("{}: found the following fields: {}", s, vec);

        debug!("{}: looking for arch in {}", s, vec[0]);
        package.arch = match vec[0] {
            "i686"   => Arch::i686,
            "x86_64" => Arch::x86_64,
            "any"    => Arch::any,
            _        => return None
        };
        debug!("{}: package arch is {}", s, package.arch);

        debug!("{}: looking for pkgrel in {}", s, vec[1]);
        package.pkgrel = match from_str::<uint>(vec[1]) {
            None    => {
                debug!("{}: invalid pkgrel: must be a positive integer", s);
                return None
            },
            Some(x) => {
                if x == 0 {
                    debug!("{}: invalid pkgrel: must be > 0", s);
                    return None
                } else {
                    x
                }
            }
        };
        debug!("{}: package pkgrel is {}", s, package.pkgrel);

        debug!("{}: looking for 'epoch:pkgver' in {}", s, vec[2]);
        let version: Vec<&str> = vec[2].splitn(2, ':').collect();
        if version.is_empty() {
            debug!("{}: empty version field", s);
            return None
        } else if version.len() == 1 {
            package.pkgver = match from_str(vec[2]) {
                None    => { // FIXME
                    debug!("{}: invalid pkgver field", s);
                    return None
                },
                Some(s) => s
            };
            debug!("{}: package pkgver is {}", s, package.pkgver);
        } else {
            package.epoch  = match from_str(version[0]) {
                None    => { // FIXME
                    debug!("{}: invalid epoch field", s);
                    return None
                },
                Some(x) => x
            };
            debug!("{}: package epoch is {}", s, package.epoch);
            package.pkgver = match from_str(version[1]) {
                None    => { // FIXME
                    debug!("{}: invalid pkgver field", s);
                    return None
                },
                Some(s) => s
            };
            debug!("{}: package pkgver is {}", s, package.pkgver);
        }

        // Everything else is the pkgname
        debug!("{}: looking for pkgname in {}", s, vec[3]);
        package.pkgname = match from_str(vec[3]) {
            None    => { // FIXME
                debug!("{}: invalid pkgname field", s);
                return None
            },
            Some(s) => s
        };
        debug!("{}: package pkgname is {}", s, package.pkgname);

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
    let package2 = Package::new("lnav", "0.5.1", 1, 0, Arch::x86_64, true, false);
    assert_eq!(package1, package2);
}

#[test]
fn package_format() {
    let filename = "lnav-0.5.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("lnav", "0.5.1", 1, 0, Arch::x86_64, true, false);
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
    let package2 = Package::new("docker", "1.3.1", 1, 1, Arch::x86_64, true, false);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_format() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz";
    let package = Package::new("docker", "1.3.1", 1, 1, Arch::x86_64, true, false);
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
    let package2 = Package::new("docker", "1.3.1", 1, 1, Arch::x86_64, false, true);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_and_sign_format() {
    let filename = "docker-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package = Package::new("docker", "1.3.1", 1, 1, Arch::x86_64, false, true);
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
    let package2 = Package::new("docker-test-foo", "1.3.1", 1, 1, Arch::x86_64, false, true);
    assert_eq!(package1, package2);
}

#[test]
fn package_with_epoch_dash_and_sign_format() {
    let filename = "docker-test-foo-1:1.3.1-1-x86_64.pkg.tar.xz.sig";
    let package = Package::new("docker-test-foo", "1.3.1", 1, 1, Arch::x86_64, false, true);
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

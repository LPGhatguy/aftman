mod github;

use std::env::consts::{ARCH, OS};

use semver::Version;

pub use self::github::GitHubSource;

#[derive(Debug)]
pub struct Release {
    pub version: Version,
    pub prerelease: bool,
    pub assets: Vec<Asset>,
}

#[derive(Debug)]
pub struct Asset {
    pub name: String,
    pub url: String,
    pub os: Option<OperatingSystem>,
    pub arch: Option<Architecture>,
    pub toolchain: Option<Toolchain>,
}

impl Asset {
    /// Tells whether this asset is definitely compatible with the current host.
    #[rustfmt::skip]
    pub fn compatible(&self) -> bool {
        use OperatingSystem as Os;
        use Architecture as Arch;

        match (self.os, self.arch) {
            (Some(Os::Windows), Some(Arch::X64)) => OS == "windows" && ARCH == "x86_64",
            (Some(Os::Windows), Some(Arch::X86)) => OS == "windows" && (ARCH == "x86_64" || ARCH == "x86"),
            (Some(Os::MacOS), Some(Arch::Arm64)) => OS == "macos" && ARCH == "aarch64",
            (Some(Os::MacOS), Some(Arch::X64)) => OS == "macos" && (ARCH == "aarch64" || ARCH == "x86_64"),
            (Some(Os::Linux), Some(Arch::X64)) => OS == "linux" && ARCH == "x86_64",
            _ => false,
        }
    }

    pub fn from_name_url(name: &str, url: &str) -> Self {
        let match_name = name.to_ascii_lowercase();

        let os = if match_name.contains("windows")
            || match_name.contains("win32")
            || match_name.contains("win64")
        {
            Some(OperatingSystem::Windows)
        } else if match_name.contains("macos") || match_name.contains("osx") {
            Some(OperatingSystem::MacOS)
        } else if match_name.contains("linux") || match_name.contains("ubuntu") {
            Some(OperatingSystem::Linux)
        } else {
            None
        };

        let arch = if match_name.contains("x86-64")
            || match_name.contains("x86_64")
            || match_name.contains("x64")
            || match_name.contains("amd64")
            || match_name.contains("win64")
        {
            Some(Architecture::X64)
        } else if match_name.contains("x86")
            || match_name.contains("i686")
            || match_name.contains("win32")
        {
            Some(Architecture::X86)
        } else if match_name.contains("aarch64") || match_name.contains("arm64") {
            Some(Architecture::Arm64)
        } else if match_name.contains("arm") || match_name.contains("arm32") {
            Some(Architecture::Arm32)
        } else {
            None
        };

        let toolchain = if match_name.contains("msvc") {
            Some(Toolchain::Msvc)
        } else if match_name.contains("musl") {
            Some(Toolchain::Musl)
        } else if match_name.contains("gnu") {
            Some(Toolchain::Gnu)
        } else {
            None
        };

        Self {
            name: name.to_owned(),
            url: url.to_owned(),
            os,
            arch,
            toolchain,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatingSystem {
    Windows,
    MacOS, // aka OS X
    Linux,
}

impl OperatingSystem {
    pub fn compatible(&self) -> bool {
        match self {
            Self::Windows => OS == "windows",
            Self::MacOS => OS == "macos",
            Self::Linux => OS == "linux",
        }
    }

    pub fn native() -> Option<Self> {
        if cfg!(target_os = "windows") {
            Some(OperatingSystem::Windows)
        } else if cfg!(target_os = "macos") {
            Some(OperatingSystem::MacOS)
        } else if cfg!(target_os = "linux") {
            Some(OperatingSystem::Linux)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Architecture {
    X86, // aka i686
    X64, // aka x86-64, AMD64
    Arm32,
    Arm64, // aka AArch64
}

impl Architecture {
    pub fn native() -> Option<Self> {
        if cfg!(target_arch = "x86") {
            Some(Architecture::X86)
        } else if cfg!(target_arch = "x86_64") {
            Some(Architecture::X64)
        } else if cfg!(target_arch = "arm") {
            Some(Architecture::Arm32)
        } else if cfg!(target_arch = "aarch64") {
            Some(Architecture::Arm64)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Toolchain {
    Msvc,
    Gnu,
    Musl,
}

impl Toolchain {
    pub fn native() -> Option<Self> {
        if cfg!(target_env = "msvc") {
            Some(Toolchain::Msvc)
        } else if cfg!(target_env = "gnu") {
            Some(Toolchain::Gnu)
        } else if cfg!(target_env = "musl") {
            Some(Toolchain::Musl)
        } else {
            None
        }
    }
}

use std::env;
use std::path::PathBuf;

use anyhow::{bail, Result};

pub(crate) type Shell = Box<dyn UnixShell>;

fn has_cmd(cmd: &str) -> bool {
    let cmd = format!("{}{}", cmd, env::consts::EXE_SUFFIX);
    let path = env::var_os("PATH").unwrap_or_default();
    env::split_paths(&path)
        .map(|p| p.join(&cmd))
        .any(|p| p.exists())
}

fn find_cmd<'a>(cmds: &[&'a str]) -> Option<&'a str> {
    cmds.iter().cloned().find(|&s| has_cmd(s))
}

fn enumerate_shells() -> Vec<Shell> {
    vec![Box::new(Posix), Box::new(Bash), Box::new(Zsh)]
}

pub(crate) fn get_available_shells() -> impl Iterator<Item = Shell> {
    enumerate_shells().into_iter().filter(|sh| sh.does_exist())
}

pub(crate) trait UnixShell {
    // Detects if a shell "exists". Users have multiple shells, so an "eager"
    // heuristic should be used, assuming shells exist if any traces do.
    fn does_exist(&self) -> bool;

    // Gives all rcfiles of a given shell that Aftman is concerned with.
    // Used primarily in checking rcfiles for cleanup.
    fn rcfiles(&self) -> Vec<PathBuf>;

    // Gives rcs that should be written to.
    fn update_rcs(&self) -> Vec<PathBuf>;

    fn source_string(&self, path: &std::path::Path) -> Result<String> {
        Ok(format!(r#". "{}/env""#, path.display()))
    }
}

struct Posix;
impl UnixShell for Posix {
    fn does_exist(&self) -> bool {
        true
    }

    fn rcfiles(&self) -> Vec<PathBuf> {
        match dirs::home_dir() {
            Some(dir) => vec![dir.join(".profile")],
            _ => vec![],
        }
    }

    fn update_rcs(&self) -> Vec<PathBuf> {
        // Write to .profile even if it doesn't exist. It's the only rc in the
        // POSIX spec so it should always be set up.
        self.rcfiles()
    }
}

struct Bash;

impl UnixShell for Bash {
    fn does_exist(&self) -> bool {
        !self.update_rcs().is_empty()
    }

    fn rcfiles(&self) -> Vec<PathBuf> {
        // Bash also may read .profile, however Aftman already includes handling
        // .profile as part of POSIX and always does setup for POSIX shells.
        [".bash_profile", ".bash_login", ".bashrc"]
            .iter()
            .filter_map(|rc| dirs::home_dir().map(|dir| dir.join(rc)))
            .collect()
    }

    fn update_rcs(&self) -> Vec<PathBuf> {
        self.rcfiles()
            .into_iter()
            .filter(|rc| rc.is_file())
            .collect()
    }
}

struct Zsh;

impl Zsh {
    fn zdotdir() -> Result<PathBuf> {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        if matches!(env::var("SHELL"), Ok(sh) if sh.contains("zsh")) {
            match env::var("ZDOTDIR") {
                Ok(dir) if !dir.is_empty() => Ok(PathBuf::from(dir)),
                _ => bail!("Zsh setup failed."),
            }
        } else {
            match std::process::Command::new("zsh")
                .args(&["-c", "'echo $ZDOTDIR'"])
                .output()
            {
                Ok(io) if !io.stdout.is_empty() => Ok(PathBuf::from(OsStr::from_bytes(&io.stdout))),
                _ => bail!("Zsh setup failed."),
            }
        }
    }
}

impl UnixShell for Zsh {
    fn does_exist(&self) -> bool {
        // zsh has to either be the shell or be callable for zsh setup.
        matches!(env::var("SHELL"), Ok(sh) if sh.contains("zsh"))
            || matches!(find_cmd(&["zsh"]), Some(_))
    }

    fn rcfiles(&self) -> Vec<PathBuf> {
        [Zsh::zdotdir().ok(), dirs::home_dir()]
            .iter()
            .filter_map(|dir| dir.as_ref().map(|p| p.join(".zshenv")))
            .collect()
    }

    fn update_rcs(&self) -> Vec<PathBuf> {
        // zsh can change $ZDOTDIR both _before_ AND _during_ reading .zshenv,
        // so we: write to $ZDOTDIR/.zshenv if-exists ($ZDOTDIR changes before)
        // OR write to $HOME/.zshenv if it exists (change-during)
        // if neither exist, we create it ourselves, but using the same logic,
        // because we must still respond to whether $ZDOTDIR is set or unset.
        // In any case we only write once.
        self.rcfiles()
            .into_iter()
            .filter(|env| env.is_file())
            .chain(self.rcfiles().into_iter())
            .take(1)
            .collect()
    }
}

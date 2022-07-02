use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

#[derive(Debug)]
pub struct Output {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub struct Environment {
    exe: String,
    temp: TempDir,
}

impl Environment {
    pub fn new(exe: &str) -> Self {
        let exe = exe.to_owned();
        let temp = TempDir::new().unwrap();

        Self { exe, temp }
    }

    pub fn path(&self) -> &Path {
        self.temp.path()
    }

    pub fn run(&self, args: &[&str]) -> Output {
        let output = Command::new(&self.exe)
            .env("AFTMAN_PATH_UNIX", "1")
            .env("AFTMAN_HOME", self.path())
            .args(args)
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        Output {
            code: output.status.code().unwrap(),
            stdout,
            stderr,
        }
    }

    pub fn breakpoint(self) {
        let path = self.temp.path().to_owned();
        std::mem::forget(self);

        panic!("Breakpoint: state at {}", path.display());
    }
}

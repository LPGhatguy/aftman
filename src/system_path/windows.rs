use std::path::Path;

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

pub fn add(path: &Path) -> anyhow::Result<bool> {
    let canonical_path = path.canonicalize()?;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment")?; // create_subkey opens with write permissions
    let global_path: String = env.get_value("PATH")?;

    for entry in global_path.split(';') {
        if let Ok(entry) = Path::new(entry).canonicalize() {
            if entry == canonical_path {
                return Ok(false);
            }
        }
    }

    let new_global_path = format!("{global_path};{}", path.display());
    env.set_value("PATH", &new_global_path)?;

    Ok(true)
}

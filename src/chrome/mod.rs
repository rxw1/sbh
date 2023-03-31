use std::path::PathBuf;

pub mod tab;
pub mod window;

// TODO Don't hardcode
pub const USER_AGENT_LINUX: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36";
pub const USER_AGENT_MACOS: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36";
pub const USER_AGENT_WINDOWS: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.1661.41";
pub const USER_AGENT_WHATEVER: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36";

/// Returns the common os dependend path where Chrome
/// extensions are stored. May return None if no
/// directory could be figured out.
pub fn get_path() -> Option<PathBuf> {
    match whoami::platform() {
        whoami::Platform::Linux => {
            if wsl::is_wsl() {
                Some(PathBuf::from(format!(
                    "/mnt/c/Users/{}/AppData/Local",
                    whoami::realname()
                )))
            } else {
                dirs::config_dir()
            }
        }
        whoami::Platform::MacOS => dirs::data_local_dir(),
        whoami::Platform::Windows => dirs::data_local_dir(),
        _ => {
            todo!("Operating system not supported");
        }
    }
}

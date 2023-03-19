pub mod backup;
pub mod session;
pub mod settings;
pub mod undo;

use std::error::Error;
use std::path::PathBuf;
use std::process::exit;

use globwalk::DirEntry;

pub const SESSION_BUDDY_APPID: &str = "edacconmaakjimmfgnblocblbcdcpbko";
pub const SESSION_BUDDY_FORMAT: &str = "nxs.json.v1";
pub const SESSION_BUDDY_VERSION: &str = "3.6.4";

pub fn get_path() -> Option<PathBuf> {
    match whoami::platform() {
        whoami::Platform::Linux => match wsl::is_wsl() {
            true => Some(PathBuf::from(format!(
                "/mnt/c/Users/{}/AppData/Local",
                whoami::realname()
            ))),
            false => dirs::config_dir(),
        },
        whoami::Platform::MacOS => dirs::data_local_dir(),
        whoami::Platform::Windows => dirs::data_local_dir(),
        _ => {
            eprintln!("Operating system not supported");
            exit(1);
        }
    }
}

pub fn find_databases(basedir: &PathBuf) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        basedir,
        &[format!(
            "/*/*/User Data/Default/databases/chrome-extension_{}_*/*",
            crate::session_buddy::SESSION_BUDDY_APPID
        )],
    )
    .max_depth(crate::SBH_MAX_WALK_DEPTH) // min 7
    .follow_links(true)
    .build()
    .unwrap()
    .filter_map(Result::ok);

    Ok(walker.collect::<Vec<DirEntry>>())
}

fn get_user_agent() -> String {
    match whoami::platform() {
        whoami::Platform::Windows => crate::USER_AGENT_WINDOWS,
        whoami::Platform::Linux => crate::USER_AGENT_LINUX,
        whoami::Platform::MacOS => crate::USER_AGENT_MACOS,
        _ => crate::USER_AGENT_WHATEVER,
    }
    .to_string()
}

fn get_platform() -> String {
    let p = whoami::platform();
    match p {
        whoami::Platform::Windows => "Win32".to_string(),
        _ => p.to_string(),
    }
}

fn get_language() -> String {
    whoami::lang().next().unwrap_or("en-US".to_string())
}

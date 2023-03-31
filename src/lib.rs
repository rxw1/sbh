pub mod args;
pub mod chrome;
pub mod session_buddy;
pub mod types;

pub const BOM: &str = "\u{FEFF}";

pub const SBH_MAX_WALK_DEPTH: usize = 8;

pub mod util {
    use std::fs::Metadata;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};

    use chrono::DateTime;
    use chrono::Datelike;
    use chrono::Local;
    use chrono::Timelike;
    use rand::distributions::Alphanumeric;
    use rand::thread_rng;
    use rand::Rng;

    pub fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    /// Returns a duration that can be used as timestamp to
    /// measure something.
    pub fn ts() -> Duration {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
    }

    pub fn get_user_agent() -> String {
        match whoami::platform() {
            whoami::Platform::Windows => crate::chrome::USER_AGENT_WINDOWS,
            whoami::Platform::Linux => crate::chrome::USER_AGENT_LINUX,
            whoami::Platform::MacOS => crate::chrome::USER_AGENT_MACOS,
            _ => crate::chrome::USER_AGENT_WHATEVER
        }
        .to_string()
    }

    pub fn get_platform() -> String {
        let p = whoami::platform();
        match p {
            whoami::Platform::Windows => "Win32".to_string(),
            _ => p.to_string()
        }
    }

    pub fn get_language() -> String {
        whoami::lang().next().unwrap_or("en-US".to_string())
    }

    pub fn generate_gid() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }

    /// Returns the last modification time listed in the
    /// metadata of a file as a formatted String.
    pub fn get_mtime_fmt(path: &Path) -> Result<String, std::io::Error> {
        let metadata: Metadata = path.metadata()?;
        let ts: DateTime<Local> = metadata.modified()?.into();
        let ts_fmt: String = format!(
            "{:0>4}_{:0>2}_{:0>2}_{:0>2}_{:0>2}_{:0>2}",
            ts.year(),
            ts.month(),
            ts.day(),
            ts.hour(),
            ts.minute(),
            ts.second()
        );
        Ok(ts_fmt)
    }

    pub fn get_output_filename(db: &Path, out: Option<PathBuf>) -> Result<PathBuf, std::io::Error> {
        match out {
            Some(p) => {
                if p.is_dir() {
                    let t = get_mtime_fmt(db)?;
                    let f = format!("session_buddy_{}.db", t);
                    Ok(p.join(f))
                } else {
                    Ok(p)
                }
            }
            None => {
                let t = get_mtime_fmt(db)?;
                let f = format!("session_buddy_{}.db", t);
                Ok(f.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use regex::Regex;

    use crate::util::get_mtime_fmt;

    const TS_REGEX: &str = r"^[0-9]{4}_[0-9]{2}_[0-9]{2}_[0-9]{2}_[0-9]{2}_[0-9]{2}$";

    #[test]
    fn ts_regex() {
        let re = Regex::new(TS_REGEX).unwrap();
        let ts = String::from("2023_03_20_04_38_33");
        assert!(re.is_match(&ts));
    }

    #[test]
    fn get_mtime_fmt_works() {
        let re = Regex::new(TS_REGEX).unwrap();
        let ts = get_mtime_fmt(&PathBuf::from("./Cargo.toml")).unwrap();
        println!("{:#?}", ts);
        assert!(re.is_match(&ts));
    }
}

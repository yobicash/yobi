use libyobicash::utils::version::YVersion;
use ::VERSION;

pub fn default_version() -> YVersion {
    YVersion::from_str(VERSION).unwrap()
}

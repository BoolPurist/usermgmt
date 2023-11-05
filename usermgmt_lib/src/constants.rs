use const_format::concatcp;
pub const SSH_TIME_OUT_MILL_SECS: u32 = 3000;
/// Name of the file in which all values for configuration of this app are located
/// besides the CLI arguments.
pub const NAME_CONFIG_FILE: &str = "conf.toml";
pub const README_LINK: &str = "https://github.com/th-nuernberg/usermgmt/blob/main/README.md";
pub const ISSUE_LINK: &str = "https://github.com/th-nuernberg/usermgmt/issues";
pub const REPOSITORY_LINK: &str = "https://github.com/th-nuernberg/usermgmt";
pub const MIT_LINK: &str = "https://github.com/th-nuernberg/usermgmt/blob/main/LICENSE";
pub const BUG_REPORT: &str = concatcp!(
    "This is a bug. If you see this message as an user of this application,
please report this bug as an issue on the issue page of this application.
Link to issue page: ",
    ISSUE_LINK,
    " ."
);

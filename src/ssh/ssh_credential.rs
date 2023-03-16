use once_cell::unsync::OnceCell;

use crate::config::config::MgmtConfig;

/// Fetches username and password lazy at the first time.
/// The fetching of username and password happens only once !
pub struct SshCredential<'a> {
    default_ssh_user: &'a str,
    username_password: OnceCell<(String, String)>,
}

impl<'a> SshCredential<'a> {
    pub fn new(config: &'a MgmtConfig) -> Self {
        Self {
            username_password: OnceCell::new(),
            default_ssh_user: &config.default_ssh_user,
        }
    }
    /// Returns given username of user or the default user name if the user has given no username
    pub fn username(&self) -> &str {
        let (username, _) = self
            .username_password
            .get_or_init(|| super::ask_credentials_for_ssh(self.default_ssh_user));

        username
    }
    pub fn password(&self) -> &str {
        let (_, password) = self
            .username_password
            .get_or_init(|| super::ask_credentials_for_ssh(self.default_ssh_user));

        password
    }
}
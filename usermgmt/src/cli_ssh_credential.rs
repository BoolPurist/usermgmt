use log::info;
use once_cell::sync::OnceCell;
use usermgmt_lib::prelude::*;

use usermgmt_lib::{
    config::MgmtConfig,
    prelude::{anyhow, AppResult},
    ssh::SshCredentials,
};

use crate::user_input;

#[derive(Debug, Clone)]
pub struct CliSshCredential {
    default_ssh_user: String,
    username: OnceCell<String>,
    password: OnceCell<String>,
}

impl CliSshCredential {
    pub fn new(config: &MgmtConfig) -> Self {
        Self {
            username: Default::default(),
            password: Default::default(),
            default_ssh_user: config.default_ssh_user.clone(),
        }
    }
}

impl SshCredentials for CliSshCredential {
    /// Returns given username of user or the default user name if the user has given no username
    fn username(&self) -> AppResult<&str> {
        let username = self.username.get_or_try_init(|| {
            user_input::ask_for_line_from_user_over_term(
                "Enter your SSH username",
                Some(self.default_ssh_user.as_str()),
            )
        })?;

        Ok(username)
    }
    fn password(&self) -> AppResult<&str> {
        let password = self.password.get_or_try_init(|| {
            let maybe_password = user_input::cli_ask_for_password("Enter your SSH password: ")?;
            maybe_password.ok_or_else(|| anyhow!("No password provided"))
        })?;

        Ok(password)
    }

    fn auth_agent_resolve(
        &self,
        many_keys: Vec<usermgmt_lib::ssh::SshPublicKeySuggestion>,
    ) -> AppResult<usize> {
        let length = many_keys.len();
        let last_index = length.saturating_sub(1);
        println!("Found more than one key in ssh agent !");
        println!("Chooose one between {} and {} ssh key", 0, last_index);
        println!("===========================================");

        for (index, next) in many_keys.iter().enumerate() {
            let comment = next.comment();
            println!("{} => comment: {}", index, comment);
        }

        let user_choice: usize = user_input::line_input_from_user()?
            .ok_or_else(|| anyhow!("No number supplied"))?
            .parse()?;

        if last_index < user_choice {
            bail!("Choice should between {} and {}", 0, last_index);
        } else {
            info!("{}. ssh key is chosen", user_choice);
            Ok(last_index)
        }
    }
}
use crate::prelude::*;

use super::query_io_tasks;
use super::top_level_drawing;

use drawing::draw_utils::GroupDrawing;
use eframe::egui::RichText;
use eframe::epaint::Color32;
use std::convert::AsRef;
use std::path::PathBuf;
use usermgmt_lib::{ldap::LdapSimpleCredential, ssh::SshGivenCredential};

use crate::current_selected_view::ModifyState;
use crate::current_selected_view::{ListingState, RemoveState, SshConnectionState};

#[cfg(debug_assertions)]
use super::settings::DebugSettingWatcher;

use crate::{
    current_selected_view::{AddState, ConfigurationState, LdapConnectionState},
    drawing::draw_utils,
    which_systems::WhichSystem,
};

#[derive(Debug)]
/// The global state of the GUI.
/// The default `impl` of this struct initializes the global state of the GUI.
/// The update function executes every frame and draws the GUI from its global state hence the
/// word `Window` in the name.
pub struct UsermgmtWindow {
    pub selected_view: CurrentSelectedView,
    pub conf_path: PathBuf,
    pub conf_state: ConfigurationState,
    pub listin_state: ListingState,
    pub ssh_state: SshConnectionState,
    pub ldap_state: LdapConnectionState,
    pub which_sys: WhichSystem,
    pub adding_state: AddState,
    pub remove_state: RemoveState,
    pub modify_state: ModifyState,
    pub settings: Settings,
    pub init: Init,
    #[cfg(debug_assertions)]
    pub settings_watcher: DebugSettingWatcher,
}

impl Default for UsermgmtWindow {
    fn default() -> Self {
        let mut conf_state: ConfigurationState = Default::default();

        info!("Loading init data for gui.");
        let init = toml::from_str(include_str!("../../assets/Init.toml"))
            .expect("Failed to init file (Init.toml).\nThis file is needed for knowing how to draw the GUI.");

        info!("Loading settings for gui.");
        let settings = toml::from_str(include_str!("../../assets/Settings.toml"))
            .expect("Failed to parse file (Settings.toml).\nThis file is needed for knowing how to draw the GUI.");

        general_utils::start_load_config(&mut conf_state, None);

        Self {
            listin_state: Default::default(),
            selected_view: Default::default(),
            ssh_state: Default::default(),
            conf_path: Default::default(),
            which_sys: Default::default(),
            ldap_state: Default::default(),
            adding_state: Default::default(),
            remove_state: Default::default(),
            modify_state: Default::default(),
            init,
            settings,
            conf_state,
            // Activate reload feature to see changes of GUI settings during development
            #[cfg(debug_assertions)]
            settings_watcher: Default::default(),
        }
    }
}

impl UsermgmtWindow {
    pub fn selected_view(&self) -> CurrentSelectedView {
        self.selected_view
    }

    pub fn set_selected_view(&mut self, selected_view: CurrentSelectedView) {
        self.selected_view = selected_view;
    }

    pub fn conf_path_owned(&self) -> String {
        self.conf_path.to_string_lossy().to_string()
    }
    pub fn set_conf_path(&mut self, new: impl Into<PathBuf>) {
        self.conf_path = new.into();
    }

    pub fn is_ssh_cred_needed(&self, supporsts_dir: bool) -> bool {
        self.which_sys.is_ssh_cred_needed(supporsts_dir)
    }
    pub fn is_ldap_needed(&self) -> bool {
        self.which_sys.is_ldap_needed()
    }

    /// Use this to get the credentials from user before the ssh connection is initialized.
    pub fn create_ssh_credentials(&self) -> Option<SshGivenCredential> {
        if let IoTaskStatus::Successful(conf) = self.conf_state.io_conf.status() {
            let ssh_state = &self.ssh_state;
            let (username, password) = (ssh_state.username.as_ref(), ssh_state.password.as_deref());
            let cred = SshGivenCredential::new(
                username?,
                password.unwrap_or_default(),
                usermgmt_lib::ssh::create_ssh_key_pair_conf(ssh_state.ssh_key_pair(), &conf.config),
            );
            Some(cred)
        } else {
            None
        }
    }

    /// Use this to get the credentials from user before a connection to the LDAP is initialized.
    pub fn create_ldap_credentials(&self) -> Option<LdapSimpleCredential> {
        let ldap_state = &self.ldap_state;
        let (username, password) = (ldap_state.username.as_ref(), ldap_state.password.as_ref());
        let cred = LdapSimpleCredential::new(username?.to_owned(), password?.to_owned());
        Some(cred)
    }
}

impl eframe::App for UsermgmtWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            query_io_tasks::query(self);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    draw_utils::draw_box_group(
                        ui,
                        &self.settings,
                        &GroupDrawing::new("Actions"),
                        |ui| {
                            ui_action_menu(
                                &self.settings,
                                &mut self.conf_state,
                                &mut self.selected_view,
                                ui,
                            )
                        },
                    )
                });
                ui.vertical(|ui| top_level_drawing::draw_selected_view(self, ui));
            });
        });
    }
}

/// Draw menu to the left of the current view.
/// Menu consists of buttons. Every button represents a view.
/// Clicking on one button changes to its respective view.
fn ui_action_menu(
    settings: &Settings,
    conf_state: &mut ConfigurationState,
    selected_view: &mut CurrentSelectedView,
    ui: &mut egui::Ui,
) {
    for next in CurrentSelectedView::iter() {
        let button_color =
            if next == CurrentSelectedView::Configuration && !conf_state.io_conf.is_there() {
                settings.colors().err_msg()
            } else {
                Color32::WHITE
            };
        if ui
            .button(RichText::new(next.as_ref()).color(button_color))
            .clicked()
        {
            let previous_view = *selected_view;
            info!("Changed from ({:?}) to ({:?}) view", previous_view, next);
            *selected_view = next;
        }
    }
}

//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// locals
use super::{
    SetupActivity, COMPONENT_INPUT_FILE_FMT, COMPONENT_INPUT_SSH_HOST,
    COMPONENT_INPUT_SSH_USERNAME, COMPONENT_INPUT_TEXT_EDITOR, COMPONENT_LIST_SSH_KEYS,
    COMPONENT_RADIO_DEFAULT_PROTOCOL, COMPONENT_RADIO_DEL_SSH_KEY, COMPONENT_RADIO_GROUP_DIRS,
    COMPONENT_RADIO_HIDDEN_FILES, COMPONENT_RADIO_QUIT, COMPONENT_RADIO_SAVE,
    COMPONENT_RADIO_UPDATES, COMPONENT_TEXT_ERROR, COMPONENT_TEXT_HELP,
};
use crate::ui::layout::{Msg, Payload};
// ext
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// -- keymap
const MSG_KEY_ENTER: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Enter,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_ESC: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Esc,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_TAB: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Tab,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_DOWN: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Down,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_UP: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Up,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_DEL: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Delete,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_CTRL_E: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_H: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('h'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_N: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('n'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_R: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('r'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_S: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('s'),
    modifiers: KeyModifiers::CONTROL,
});

// -- update

impl SetupActivity {
    /// ### update
    ///
    /// Update auth activity model based on msg
    /// The function exits when returns None
    pub(super) fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = match msg.as_ref() {
            None => None,
            Some((s, msg)) => Some((s, msg)),
        };
        // Match msg
        match ref_msg {
            None => None,
            Some(msg) => match msg {
                // Input field <DOWN>
                (COMPONENT_INPUT_TEXT_EDITOR, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_RADIO_DEFAULT_PROTOCOL);
                    None
                }
                (COMPONENT_RADIO_DEFAULT_PROTOCOL, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_RADIO_HIDDEN_FILES);
                    None
                }
                (COMPONENT_RADIO_HIDDEN_FILES, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_RADIO_UPDATES);
                    None
                }
                (COMPONENT_RADIO_UPDATES, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_RADIO_GROUP_DIRS);
                    None
                }
                (COMPONENT_RADIO_GROUP_DIRS, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_INPUT_FILE_FMT);
                    None
                }
                (COMPONENT_INPUT_FILE_FMT, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_INPUT_TEXT_EDITOR);
                    None
                }
                // Input field <UP>
                (COMPONENT_INPUT_FILE_FMT, &MSG_KEY_UP) => {
                    self.view.active(COMPONENT_RADIO_GROUP_DIRS);
                    None
                }
                (COMPONENT_RADIO_GROUP_DIRS, &MSG_KEY_UP) => {
                    self.view.active(COMPONENT_RADIO_UPDATES);
                    None
                }
                (COMPONENT_RADIO_UPDATES, &MSG_KEY_UP) => {
                    self.view.active(COMPONENT_RADIO_HIDDEN_FILES);
                    None
                }
                (COMPONENT_RADIO_HIDDEN_FILES, &MSG_KEY_UP) => {
                    self.view.active(COMPONENT_RADIO_DEFAULT_PROTOCOL);
                    None
                }
                (COMPONENT_RADIO_DEFAULT_PROTOCOL, &MSG_KEY_UP) => {
                    self.view.active(COMPONENT_INPUT_TEXT_EDITOR);
                    None
                }
                (COMPONENT_INPUT_TEXT_EDITOR, &MSG_KEY_UP) => {
                    self.view.active(COMPONENT_INPUT_FILE_FMT);
                    None
                }
                // Error <ENTER> or <ESC>
                (COMPONENT_TEXT_ERROR, &MSG_KEY_ENTER) | (COMPONENT_TEXT_ERROR, &MSG_KEY_ESC) => {
                    // Umount text error
                    self.umount_error();
                    None
                }
                // Exit
                (COMPONENT_RADIO_QUIT, Msg::OnSubmit(Payload::Unsigned(0))) => {
                    // Save changes
                    if let Err(err) = self.action_save_config() {
                        self.mount_error(err.as_str());
                    }
                    // Exit
                    self.quit = true;
                    None
                }
                (COMPONENT_RADIO_QUIT, Msg::OnSubmit(Payload::Unsigned(1))) => {
                    // Quit
                    self.quit = true;
                    self.umount_quit();
                    None
                }
                (COMPONENT_RADIO_QUIT, Msg::OnSubmit(_)) => {
                    // Umount popup
                    self.umount_quit();
                    None
                }
                // Close help
                (COMPONENT_TEXT_HELP, &MSG_KEY_ENTER) | (COMPONENT_TEXT_HELP, &MSG_KEY_ESC) => {
                    // Umount help
                    self.umount_help();
                    None
                }
                // Delete key
                (COMPONENT_RADIO_DEL_SSH_KEY, Msg::OnSubmit(Payload::Unsigned(0))) => {
                    // Delete key
                    self.action_delete_ssh_key();
                    // Reload ssh keys
                    self.reload_ssh_keys();
                    // Delete popup
                    self.umount_del_ssh_key();
                    None
                }
                (COMPONENT_RADIO_DEL_SSH_KEY, Msg::OnSubmit(_)) => {
                    // Umount
                    self.umount_del_ssh_key();
                    None
                }
                // Save popup
                (COMPONENT_RADIO_SAVE, Msg::OnSubmit(Payload::Unsigned(0))) => {
                    // Save config
                    if let Err(err) = self.action_save_config() {
                        self.mount_error(err.as_str());
                    }
                    self.umount_save_popup();
                    None
                }
                (COMPONENT_RADIO_SAVE, Msg::OnSubmit(_)) => {
                    // Umount radio save
                    self.umount_save_popup();
                    None
                }
                // Edit SSH Key
                // <TAB> Change view
                (COMPONENT_LIST_SSH_KEYS, &MSG_KEY_TAB) => {
                    // Change view
                    self.init_setup();
                    None
                }
                // <CTRL+H> Show help
                (_, &MSG_KEY_CTRL_H) => {
                    // Show help
                    self.mount_help();
                    None
                }
                // New key <DOWN>
                (COMPONENT_INPUT_SSH_HOST, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_INPUT_SSH_USERNAME);
                    None
                }
                (COMPONENT_INPUT_SSH_USERNAME, &MSG_KEY_DOWN) => {
                    self.view.active(COMPONENT_INPUT_SSH_HOST);
                    None
                }
                // New key <UP>
                (COMPONENT_INPUT_SSH_USERNAME, &MSG_KEY_UP) | (COMPONENT_INPUT_SSH_USERNAME, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_INPUT_SSH_HOST);
                    None
                }
                (COMPONENT_INPUT_SSH_HOST, &MSG_KEY_UP) | (COMPONENT_INPUT_SSH_HOST, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_INPUT_SSH_USERNAME);
                    None
                }
                // New key <ENTER>
                (COMPONENT_INPUT_SSH_HOST, Msg::OnSubmit(_))
                | (COMPONENT_INPUT_SSH_USERNAME, Msg::OnSubmit(_)) => {
                    // Save ssh key
                    self.action_new_ssh_key();
                    self.umount_new_ssh_key();
                    self.reload_ssh_keys();
                    None
                }
                // New key <ESC>
                (COMPONENT_INPUT_SSH_HOST, &MSG_KEY_ESC)
                | (COMPONENT_INPUT_SSH_USERNAME, &MSG_KEY_ESC) => {
                    // Umount new ssh key
                    self.umount_new_ssh_key();
                    None
                }
                // <CTRL+N> New key
                (COMPONENT_LIST_SSH_KEYS, &MSG_KEY_CTRL_N) => {
                    // Show new key popup
                    self.mount_new_ssh_key();
                    None
                }
                // <ENTER> Edit key
                (COMPONENT_LIST_SSH_KEYS, Msg::OnSubmit(Payload::Unsigned(idx))) => {
                    // Edit ssh key
                    if let Err(err) = self.edit_ssh_key(*idx) {
                        self.mount_error(err.as_str());
                    }
                    None
                }
                // <DEL | CTRL+E> Show delete
                (COMPONENT_LIST_SSH_KEYS, &MSG_KEY_CTRL_E)
                | (COMPONENT_LIST_SSH_KEYS, &MSG_KEY_DEL) => {
                    // Show delete key
                    self.mount_del_ssh_key();
                    None
                }
                (_, &MSG_KEY_TAB) => {
                    // Change view
                    self.init_ssh_keys();
                    None
                }
                // <CTRL+R> Revert changes
                (_, &MSG_KEY_CTRL_R) => {
                    // Revert changes
                    if let Err(err) = self.action_reset_config() {
                        self.mount_error(err.as_str());
                    }
                    None
                }
                // <CTRL+S> Save
                (_, &MSG_KEY_CTRL_S) => {
                    // Show save
                    self.mount_save_popup();
                    None
                }
                // <ESC>
                (_, &MSG_KEY_ESC) => {
                    // Mount quit prompt
                    self.mount_quit();
                    None
                }
                (_, _) => None, // Nothing to do
            },
        }
    }
}
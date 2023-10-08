use self::account::login::LoginDialogueState;

use super::MyHandler;
use dptree::case;
use dptree::entry;

pub mod account;

#[derive(Debug, Clone)]
pub enum DialogueState {
    LoginDialogue(LoginDialogueState),
}

impl DialogueState {
    pub fn schema() -> MyHandler {
        entry().branch(case![DialogueState::LoginDialogue(x)].chain(LoginDialogueState::schema()))
    }
}

impl From<LoginDialogueState> for DialogueState {
    fn from(state: LoginDialogueState) -> Self {
        Self::LoginDialogue(state)
    }
}

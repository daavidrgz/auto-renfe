use self::account::login::LoginDialogueState;
use self::purchase::PurchaseDialogueState;

use super::MyHandler;
use dptree::case;
use dptree::entry;

pub mod account;
pub mod purchase;

#[derive(Debug, Clone)]
pub enum DialogueState {
    LoginDialogue(LoginDialogueState),
    PurchaseDialogue(PurchaseDialogueState),
}

impl DialogueState {
    pub fn schema() -> MyHandler {
        entry()
            .branch(case![DialogueState::LoginDialogue(x)].chain(LoginDialogueState::schema()))
            .branch(
                case![DialogueState::PurchaseDialogue(x)].chain(PurchaseDialogueState::schema()),
            )
    }
}

impl From<LoginDialogueState> for DialogueState {
    fn from(state: LoginDialogueState) -> Self {
        Self::LoginDialogue(state)
    }
}

impl From<PurchaseDialogueState> for DialogueState {
    fn from(state: PurchaseDialogueState) -> Self {
        Self::PurchaseDialogue(state)
    }
}

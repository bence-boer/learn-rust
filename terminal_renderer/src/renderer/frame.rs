use crate::renderer::{TerminalAction, TerminalControlSequence};

pub fn render(actions: &[TerminalAction]) -> String {
    actions.iter().map(|action| action.to_string()).collect()
}

mod renderer;

use crate::renderer::terminal_operations::TerminalControlSequence;
use crate::renderer::{
    terminal_background_colors::TerminalBackgroundColor,
    terminal_foreground_colors::TerminalForegroundColor,
};

fn main() {
    println!("Terminal Renderer");

    print!(
        "{}{}{}{}{}{}alma alma{}",
        TerminalControlSequence::ClearScreen,
        TerminalControlSequence::MoveCursorToHome,
        TerminalBackgroundColor::CustomColor24Bit(190, 70, 210),
        TerminalForegroundColor::CustomColor24Bit(123, 200, 34),
        TerminalControlSequence::TextBold,
        TerminalControlSequence::TextItalic,
        TerminalControlSequence::ResetAllStyles
    );
}

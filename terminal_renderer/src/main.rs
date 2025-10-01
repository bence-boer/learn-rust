mod renderer;

use crate::renderer::{
    render, Coordinate, TerminalAction, TerminalBackgroundColor, TerminalControlSequence,
    TerminalForegroundColor,
};

fn main() {
    println!("Terminal Renderer");

    // print!(
    //     "{}{}{}{}{}{}alma alma{}\n",
    //     TerminalControlSequence::ClearScreen,
    //     TerminalControlSequence::MoveCursorToHome,
    //     TerminalBackgroundColor::CustomColor24Bit(190, 70, 210),
    //     TerminalForegroundColor::CustomColor24Bit(123, 200, 34),
    //     TerminalControlSequence::TextBold,
    //     TerminalControlSequence::TextItalic,
    //     TerminalControlSequence::ResetAllStyles
    // );

    println!(
        "{}",
        render(&[TerminalAction::WriteText(
            Coordinate { x: 0, y: 0 },
            "alma alma".into(),
            TextFormatOptions::default()
        )])
    )
}

use std::fmt;

use crate::renderer::{Coordinate, TerminalBackgroundColor, TerminalForegroundColor};

pub enum TerminalControlSequence {
    MoveCursorToHome,
    MoveCursorToPosition(Coordinate),
    MoveCursorUpBy(u16),
    MoveCursorDownBy(u16),
    MoveCursorForwardBy(u16),
    MoveCursorBackBy(u16),
    SaveCursorPosition,
    RestoreSavedCursorPosition,
    HideCursor,
    ShowCursor,
    ClearScreen,
    ClearFromCursorToScreenStart,
    ClearFromCursorToScreenEnd,
    ClearLine,
    ClearFromCursorToLineStart,
    ClearFromCursorToLineEnd,
    ResetAllStyles,
    TextBold,
    TextFaint,
    TextItalic,
    TextUnderline,
    TextBlinking,
    TextReverse,
    TextStrikethrough,
    TextNoBold,
    TextNoUnderline,
    TextNoBlinking,
    TextNoReverse,
    ColorForeground(TerminalForegroundColor),
    ColorBackground(TerminalBackgroundColor),
    EnterAlternateScreen,
    LeaveAlternateScreen,
}

impl fmt::Display for TerminalControlSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalControlSequence::MoveCursorToHome => {
                write!(f, "\x1B[H")
            }
            TerminalControlSequence::MoveCursorToPosition(position) => {
                write!(f, "\x1B[{};{}H", position.y, position.x)
            }
            TerminalControlSequence::MoveCursorUpBy(n) => {
                write!(f, "\x1B[{}A", n)
            }
            TerminalControlSequence::MoveCursorDownBy(n) => {
                write!(f, "\x1B[{}B", n)
            }
            TerminalControlSequence::MoveCursorForwardBy(n) => {
                write!(f, "\x1B[{}C", n)
            }
            TerminalControlSequence::MoveCursorBackBy(n) => {
                write!(f, "\x1B[{}D", n)
            }
            TerminalControlSequence::SaveCursorPosition => {
                write!(f, "\x1B[s")
            }
            TerminalControlSequence::RestoreSavedCursorPosition => {
                write!(f, "\x1B[u")
            }
            TerminalControlSequence::HideCursor => {
                write!(f, "\x1B[?25l")
            }
            TerminalControlSequence::ShowCursor => {
                write!(f, "\x1B[?25h")
            }
            TerminalControlSequence::ClearScreen => {
                write!(f, "\x1B[2J")
            }
            TerminalControlSequence::ClearFromCursorToScreenStart => {
                write!(f, "\x1B[1J")
            }
            TerminalControlSequence::ClearFromCursorToScreenEnd => {
                write!(f, "\x1B[0J")
            }
            TerminalControlSequence::ClearLine => {
                write!(f, "\x1B[2K")
            }
            TerminalControlSequence::ClearFromCursorToLineStart => {
                write!(f, "\x1B[1K")
            }
            TerminalControlSequence::ClearFromCursorToLineEnd => {
                write!(f, "\x1B[0K")
            }
            TerminalControlSequence::ResetAllStyles => {
                write!(f, "\x1B[0m")
            }
            TerminalControlSequence::TextBold => {
                write!(f, "\x1B[1m")
            }
            TerminalControlSequence::TextFaint => {
                write!(f, "\x1B[2m")
            }
            TerminalControlSequence::TextItalic => {
                write!(f, "\x1B[3m")
            }
            TerminalControlSequence::TextUnderline => {
                write!(f, "\x1B[4m")
            }
            TerminalControlSequence::TextBlinking => {
                write!(f, "\x1B[5m")
            }
            TerminalControlSequence::TextReverse => {
                write!(f, "\x1B[7m")
            }
            TerminalControlSequence::TextStrikethrough => {
                write!(f, "\x1B[9m")
            }
            TerminalControlSequence::TextNoBold => {
                write!(f, "\x1B[22m")
            }
            TerminalControlSequence::TextNoUnderline => {
                write!(f, "\x1B[24m")
            }
            TerminalControlSequence::TextNoBlinking => {
                write!(f, "\x1B[25m")
            }
            TerminalControlSequence::TextNoReverse => {
                write!(f, "\x1B[27m")
            }
            TerminalControlSequence::ColorForeground(color) => {
                write!(f, "{}", color)
            }
            TerminalControlSequence::ColorBackground(color) => {
                write!(f, "{}", color)
            }
            TerminalControlSequence::EnterAlternateScreen => {
                write!(f, "\x1B[?1049h")
            }
            TerminalControlSequence::LeaveAlternateScreen => {
                write!(f, "\x1B[?1049l")
            }
        }
    }
}

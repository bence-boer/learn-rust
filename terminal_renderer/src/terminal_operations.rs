use std::fmt;

enum TerminalControlSequence {
    MoveCursorToHome,
    MoveCursorToPosition(u16, u16),
    MoveCursorUpBy(u16),
    MoveCursorDownBy(u16),
    MoveCursorForwardBy(u16),
    MoveCursorBackBy(u16),
    SaveCursorPosition,
    RestoreSavedCursorPosition,
    HideCursor,
    ShowCursor,
    ClearScreen,
    ClearFromCursorToScreenEnd,
    ClearLine,
    ClearFromCursorToLineEnd,
    ResetAllStyles,
    TextBold,
    TextFaint,
    TextItalic,
    TextUnderline,
    CursorBlinking,
    Reverse,
}

// TODO: implement text and background coloring with parameterizable enum variant with enum parameter for color

impl fmt::Display for TerminalControlSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalControlSequence::MoveCursorToHome => {
                write!(f, "\x1B[H")
            }
            TerminalControlSequence::MoveCursorToPosition(row, col) => {
                write!(f, "\x1B[{};{}H", row, col)
            } // ... handle other variants
        }
    }
}

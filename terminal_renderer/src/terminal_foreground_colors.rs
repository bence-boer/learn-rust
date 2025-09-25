use std::fmt;

enum TerminalForegroundColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    CustomColor8Bit(u8),
    CustomColor24Bit(u8, u8, u8),
}

impl fmt::Display for TerminalForegroundColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalForegroundColor::Black => {
                write!(f, "\x1B[30m")
            }
            TerminalForegroundColor::Red => {
                write!(f, "\x1B[31m")
            }
            TerminalForegroundColor::Green => {
                write!(f, "\x1B[32m")
            }
            TerminalForegroundColor::Yellow => {
                write!(f, "\x1B[33m")
            }
            TerminalForegroundColor::Blue => {
                write!(f, "\x1B[34m")
            }
            TerminalForegroundColor::Magenta => {
                write!(f, "\x1B[35m")
            }
            TerminalForegroundColor::Cyan => {
                write!(f, "\x1B[36m")
            }
            TerminalForegroundColor::White => {
                write!(f, "\x1B[37m")
            }
            TerminalForegroundColor::BrightBlack => {
                write!(f, "\x1B[90m")
            }
            TerminalForegroundColor::BrightRed => {
                write!(f, "\x1B[91m")
            }
            TerminalForegroundColor::BrightGreen => {
                write!(f, "\x1B[92m")
            }
            TerminalForegroundColor::BrightYellow => {
                write!(f, "\x1B[93m")
            }
            TerminalForegroundColor::BrightBlue => {
                write!(f, "\x1B[94m")
            }
            TerminalForegroundColor::BrightMagenta => {
                write!(f, "\x1B[95m")
            }
            TerminalForegroundColor::BrightCyan => {
                write!(f, "\x1B[96m")
            }
            TerminalForegroundColor::BrightWhite => {
                write!(f, "\x1B[97m")
            }
            TerminalForegroundColor::CustomColor8Bit(color) => {
                write!(f, "\x1B[38;5;{}m", color)
            }
            TerminalForegroundColor::CustomColor24Bit(r, g, b) => {
                write!(f, "\x1B[38;2;{};{};{}m", r, g, b)
            }
        }
    }
}

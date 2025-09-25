use std::fmt;

enum TerminalBackgroundColor {
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

impl fmt::Display for TerminalBackgroundColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalBackgroundColor::Black => {
                write!(f, "\x1B[40m")
            }
            TerminalBackgroundColor::Red => {
                write!(f, "\x1B[41m")
            }
            TerminalBackgroundColor::Green => {
                write!(f, "\x1B[42m")
            }
            TerminalBackgroundColor::Yellow => {
                write!(f, "\x1B[43m")
            }
            TerminalBackgroundColor::Blue => {
                write!(f, "\x1B[44m")
            }
            TerminalBackgroundColor::Magenta => {
                write!(f, "\x1B[45m")
            }
            TerminalBackgroundColor::Cyan => {
                write!(f, "\x1B[46m")
            }
            TerminalBackgroundColor::White => {
                write!(f, "\x1B[47m")
            }
            TerminalBackgroundColor::BrightBlack => {
                write!(f, "\x1B[100m")
            }
            TerminalBackgroundColor::BrightRed => {
                write!(f, "\x1B[101m")
            }
            TerminalBackgroundColor::BrightGreen => {
                write!(f, "\x1B[102m")
            }
            TerminalBackgroundColor::BrightYellow => {
                write!(f, "\x1B[103m")
            }
            TerminalBackgroundColor::BrightBlue => {
                write!(f, "\x1B[104m")
            }
            TerminalBackgroundColor::BrightMagenta => {
                write!(f, "\x1B[105m")
            }
            TerminalBackgroundColor::BrightCyan => {
                write!(f, "\x1B[106m")
            }
            TerminalBackgroundColor::BrightWhite => {
                write!(f, "\x1B[107m")
            }
            TerminalBackgroundColor::CustomColor8Bit(color) => {
                write!(f, "\x1B[48;5;{}m", color)
            }
            TerminalBackgroundColor::CustomColor24Bit(r, g, b) => {
                write!(f, "\x1B[48;2;{};{};{}m", r, g, b)
            }
        }
    }
}

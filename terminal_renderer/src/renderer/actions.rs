use std::fmt;

use crate::renderer::{
    Coordinate, TerminalBackgroundColor, TerminalControlSequence, TerminalForegroundColor,
};

#[derive(Debug, Clone)]
pub enum TerminalAction {
    OutlineRectangle(Coordinate, Coordinate, TerminalBackgroundColor),
    FillRectangle(Coordinate, Coordinate, TerminalBackgroundColor),
    FillLine(Coordinate, Coordinate, TerminalBackgroundColor),
    WriteText(Coordinate, String, TextFormatOptions),
}

impl fmt::Display for TerminalAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalAction::OutlineRectangle(start, end, color) => {
                let mut output = String::new();

                // Draw top and bottom horizontal lines
                for y in [start.y, end.y] {
                    output.push_str(&format!(
                        "{}",
                        TerminalControlSequence::MoveCursorToPosition(Coordinate { x: start.x, y })
                    ));
                    output.push_str(&format!("{}", color));
                    output.push_str(&" ".repeat((end.x - start.x + 1) as usize));
                }

                // Draw left and right vertical lines
                for y in (start.y + 1)..end.y {
                    // Left edge
                    output.push_str(&format!(
                        "{}",
                        TerminalControlSequence::MoveCursorToPosition(Coordinate { x: start.x, y })
                    ));
                    output.push_str(&format!("{}", color));
                    output.push(' ');

                    // Right edge
                    output.push_str(&format!(
                        "{}",
                        TerminalControlSequence::MoveCursorToPosition(Coordinate { x: end.x, y })
                    ));
                    output.push_str(&format!("{}", color));
                    output.push(' ');
                }

                // Reset colors
                output.push_str(&format!("{}", TerminalControlSequence::ResetAllStyles));
                write!(f, "{}", output)
            }
            TerminalAction::FillRectangle(start, end, color) => {
                let mut output = String::new();
                let width = (end.x - start.x + 1) as usize;

                output.push_str(&format!("{}", color));

                for y in start.y..=end.y {
                    output.push_str(&format!(
                        "{}",
                        TerminalControlSequence::MoveCursorToPosition(Coordinate { x: start.x, y })
                    ));
                    output.push_str(&" ".repeat(width));
                }

                output.push_str(&format!("{}", TerminalControlSequence::ResetAllStyles));
                write!(f, "{}", output)
            }
            TerminalAction::FillLine(start, end, color) => {
                let width = (end.x - start.x + 1) as usize;
                write!(
                    f,
                    "{}{}{}{}",
                    TerminalControlSequence::MoveCursorToPosition(*start),
                    color,
                    " ".repeat(width),
                    TerminalControlSequence::ResetAllStyles
                )
            }
            TerminalAction::WriteText(position, text, format_options) => {
                write!(
                    f,
                    "{}{}{}{}",
                    TerminalControlSequence::MoveCursorToPosition(*position),
                    format_options.to_control_sequences(),
                    text,
                    TerminalControlSequence::ResetAllStyles
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextFormatOptions {
    background: TerminalBackgroundColor,
    color: TerminalForegroundColor,
    bold: bool,
    faint: bool,
    italic: bool,
    underline: bool,
    blinking: bool,
    reverse: bool,
    strikethrough: bool,
}

impl TextFormatOptions {
    pub fn default() -> Self {
        Self {
            background: TerminalBackgroundColor::Black,
            color: TerminalForegroundColor::White,
            bold: false,
            faint: false,
            italic: false,
            underline: false,
            blinking: false,
            reverse: false,
            strikethrough: false,
        }
    }

    pub fn to_control_sequences(&self) -> String {
        let mut sequences = String::new();

        sequences.push_str(&format!("{}", self.background));
        sequences.push_str(&format!("{}", self.color));

        if self.bold {
            sequences.push_str(&format!("{}", TerminalControlSequence::TextBold));
        }
        if self.faint {
            sequences.push_str(&format!("{}", TerminalControlSequence::TextFaint));
        }
        if self.italic {
            sequences.push_str(&format!("{}", TerminalControlSequence::TextItalic));
        }
        if self.underline {
            sequences.push_str(&format!("{}", TerminalControlSequence::TextUnderline));
        }
        if self.blinking {
            sequences.push_str(&format!("{}", TerminalControlSequence::TextBlinking));
        }
        if self.reverse {
            sequences.push_str(&format!("{}", TerminalControlSequence::TextReverse));
        }
        if self.strikethrough {
            sequences.push_str(&format!("{}", TerminalControlSequence::TextStrikethrough));
        }

        sequences
    }
}

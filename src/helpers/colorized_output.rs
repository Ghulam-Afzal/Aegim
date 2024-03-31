use ansi_term::Colour::{Cyan, Red};

pub fn colorize_output(output: String) -> String {
    if output.contains("Error") {
        Red.bold().paint(output).to_string()
    } else {
        Cyan.paint(output).to_string()
    }
}

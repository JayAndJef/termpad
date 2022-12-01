pub mod term;
use term::{TermSize, Terminal};
use crossterm::Result;

fn main() -> Result<()> {
    let mut t = Terminal::new(TermSize {width: 50, length: 20}, crossterm::style::Color::DarkGrey);
    t.init()?;
    t.mainloop()?;
    Ok(())
}

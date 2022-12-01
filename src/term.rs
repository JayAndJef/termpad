use std::io::{stdout, Stdout, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize, Color}, cursor::{self, position, MoveDown, MoveUp, MoveLeft, MoveRight, SavePosition, RestorePosition}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}, Result, event::{read, Event, KeyCode}, ExecutableCommand
};

#[derive(Clone)]
pub struct TermSize {
    pub width: u16,
    pub length: u16,
}

pub struct Terminal {
    stdout: Stdout,
    dimensions: TermSize,
    pix_array:  Vec<Color>,
}

impl Terminal {
    pub fn new(size: TermSize, background: Color) -> Self {
        Terminal { stdout: stdout(), dimensions: size.clone(), pix_array: vec![background; (size.width * size.length).into()]}
    }

    pub fn init(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All), terminal::SetSize(self.dimensions.width, self.dimensions.length), SavePosition)?;
        
        for (index, pix) in self.pix_array.iter().enumerate() {
            let x_cur = index as u16 % self.dimensions.width;
            let y_cur = index as u16 / self.dimensions.width;

            queue!(self.stdout, cursor::MoveTo(x_cur, y_cur), style::PrintStyledContent("█".with(*pix)))?;
        }

        self.stdout.flush()?;
        execute!(self.stdout, RestorePosition)?;
        Ok(())
    }

    pub fn mainloop(&mut self) -> Result<()> {
        self.update()?;
        loop {
            let current_event = read()?;
            if let Event::Key(keyevent) = current_event {
                match keyevent.code {
                    KeyCode::Char(' ') => {
                        self.change_pix(position().unwrap().0, position().unwrap().1)?;
                        self.update()?;
                    }
                    KeyCode::Esc => break,
                    KeyCode::Down => (|| {self.stdout.execute(MoveDown(1)).unwrap();})(),
                    KeyCode::Up => (|| {self.stdout.execute(MoveUp(1)).unwrap();})(),
                    KeyCode::Right => (|| {self.stdout.execute(MoveRight(1)).unwrap();})(),
                    KeyCode::Left => (|| {self.stdout.execute(MoveLeft(1)).unwrap();})(),
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    pub fn change_pix(&mut self, x_coor: u16, y_coor: u16) -> Result<()> {
        let pix = &mut self.pix_array[(y_coor as usize) * (self.dimensions.width as usize) + x_coor as usize];
        *pix = match *pix {
            Color::DarkGrey => Color::DarkRed,
            Color::DarkRed => Color::DarkGreen,
            Color::DarkGreen => Color::DarkBlue,
            Color::DarkBlue => Color::DarkYellow,
            Color::DarkYellow => Color::DarkMagenta,
            Color::DarkMagenta => Color::DarkGrey,
            Color::Rgb{..} => panic!("No rgb values allowed!"),
            _ => Color::DarkGrey,
        };
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
    }
}
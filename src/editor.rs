use std::{fs::File, io::{stdin, stdout, Write}};

use anyhow::Result;

use crossterm::{
    cursor::{
        MoveUp,
        MoveToColumn,
    }, event::{
        self,
        Event,
        KeyCode
    }, execute, terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        Clear,
        ClearType,
    }, ExecutableCommand
};

pub struct Editor {
    should_quit: bool,
    text: String,
}

impl Editor {
    pub fn new() -> Self {
        Self { 
            should_quit: false,
            text: String::new(),
        }
    }
    
    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        while !self.should_quit {
            self.process_keypress()?;
        }

        terminal::disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        print!("\nEnter file name to save: ");
        stdout().flush()?;

        let mut file_name = String::new();
        stdin().read_line(&mut file_name)?;

        let file_name = file_name.trim();

        if !file_name.is_empty() {
            let mut file = File::create(file_name)?;
            file.write_all(self.text.as_bytes())?;
            execute!(
                stdout(),
                MoveUp(1),
                Clear(ClearType::CurrentLine),
                MoveToColumn(1),
            )?;
            println!("\nFile saved successfully.");
        } else {
            execute!(
                stdout(),
                MoveUp(1),
                Clear(ClearType::CurrentLine),
                MoveToColumn(1),
            )?;
            println!("\nSave cancelled.");
        }

        Ok(())
    }

    fn process_keypress(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(c) if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) && c == 's' => {
                    self.save()?;
                },
                KeyCode::Char(c) => {
                    self.text.push(c);
                    print!("{}", c);
                    stdout().flush()?;
                },
                KeyCode::Enter => {
                    self.text.push_str("\r\n");
                    print!("\r\n");
                    stdout().flush()?;
                },
                KeyCode::Esc => self.should_quit = true,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}

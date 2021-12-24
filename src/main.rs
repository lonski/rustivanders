use std::time::Instant;
use std::{io::stdout, io::Write, time::Duration};

use crossterm::{
    cursor,
    event::{poll, read},
    event::{Event, KeyCode},
    execute, queue, style,
    style::Stylize,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    Result,
};

struct Game {
    pos: (u16, u16),
    color: style::Color,
    is_exiting: bool,
    next_tick: i128,
}

impl Game {
    fn new() -> Self {
        Game {
            pos: (0, 0),
            next_tick: 0,
            is_exiting: false,
            color: style::Color::Red,
        }
    }

    fn render<W>(&self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;
        queue!(
            w,
            cursor::MoveTo(self.pos.0, self.pos.1),
            style::Print(style::style("X").with(self.color)),
        )?;

        w.flush()
    }

    fn process_input(&mut self, event: &Event) {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(key) => match key {
                    'j' => self.pos = (self.pos.0, self.pos.1 + 1),
                    'k' => self.pos = (self.pos.0, self.pos.1 - 1),
                    'l' => self.pos = (self.pos.0 + 1, self.pos.1),
                    'h' => self.pos = (self.pos.0 - 1, self.pos.1),
                    _ => {}
                },
                KeyCode::Esc => self.is_exiting = true,
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self, dt: Duration) {
        self.next_tick -= dt.as_nanos() as i128;
        if self.next_tick <= 0 {
            match self.color {
                style::Color::Red => self.color = style::Color::Blue,
                _ => self.color = style::Color::Red,
            }
            self.next_tick = 1000 * 1000 * 200; //200 ms
        }
    }

    fn run<W>(&mut self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        enable_raw_mode()?;
        execute!(w, terminal::EnterAlternateScreen)?;

        let mut last_time = Instant::now();

        loop {
            let current_time = Instant::now();
            let elapsed = current_time - last_time;

            if poll(Duration::from_millis(1))? {
                if let Ok(event) = read() {
                    self.process_input(&event);
                }
            }

            self.update(elapsed);
            self.render(w)?;

            last_time = current_time;

            if self.is_exiting {
                break;
            }
        }

        execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;
        disable_raw_mode()?;

        Ok({})
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut game = Game::new();
    game.run(&mut stdout)
}

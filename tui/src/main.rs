use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::Print,
    terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use reginae_solver::{Board, Solution, Solver};
use std::io::{self, Write};

const QUEEN: char = '\u{2588}';
const ATTACKED: char = '\u{2593}';
const FREE: char = '\u{2591}';

#[derive(Debug)]
struct State {
    board: Board,
    messages: Vec<String>,
    pos: (u16, u16),
    stdout: io::Stdout,
}

impl State {
    pub fn new(width: usize) -> Self {
        Self {
            board: Board::new(width),
            messages: Vec::with_capacity(8),
            pos: (0, 0),
            stdout: io::stdout(),
        }
    }

    fn input(&mut self) -> io::Result<bool> {
        self.messages.clear();
        let width = self.board.width() as u16;
        let key;
        loop {
            match event::read()? {
                Event::Key(ev) if matches!(ev.kind, KeyEventKind::Press | KeyEventKind::Repeat) => {
                    key = ev.code;
                    break;
                }
                _ => (),
            }
        }
        match key {
            KeyCode::Char('q') => return Ok(false),
            KeyCode::Char('h') | KeyCode::Left => {
                self.pos.0 = self.pos.0.saturating_sub(1);
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.pos.1 = (self.pos.1 + 1).min(width - 1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.pos.1 = self.pos.1.saturating_sub(1);
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.pos.0 = (self.pos.0 + 1).min(width - 1);
            }
            KeyCode::Char(' ') => {
                self.board
                    .toggle_with_pair(self.pos.0 as usize, self.pos.1 as usize);
                if self.board.is_solved() {
                    self.messages.push("solved!".to_string());
                }
            }
            KeyCode::Char('c') => {
                self.board.clear();
            }
            KeyCode::Char('x') => {
                let board = self.board.clone();
                let Solution {
                    board,
                    success,
                    jumps,
                } = Solver::default().solve(board);
                if success {
                    self.board = board;
                    self.messages.push(format!("solved in {jumps} jumps!"));
                } else {
                    self.messages
                        .push(format!("board exhausted in {jumps} jumps!"));
                }
            }
            KeyCode::Char('r') => {
                execute!(
                    self.stdout,
                    MoveTo(0, width + 2),
                    Print("enter the new width: ")
                )?;
                disable_raw_mode()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                enable_raw_mode()?;
                input.retain(|c| c.is_ascii_digit());
                match input.parse::<u16>() {
                    Ok(w) => {
                        self.pos = (0, 0);
                        self.board = Board::new(w as usize);
                    }
                    Err(e) => self.messages.push(e.to_string()),
                }
            }
            KeyCode::Char(c) => self.messages.push(format!("unknown `{c}` command")),
            _ => (),
        }
        Ok(true)
    }

    fn render(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0)
        )?;
        let mut i = 0;
        for row in self.board.rows() {
            let mut j = 0;
            row.iter().try_for_each(|c| {
                let c = if c.is_queen() {
                    QUEEN
                } else if c.is_attacked() {
                    ATTACKED
                } else {
                    FREE
                };
                queue!(self.stdout, MoveTo(j, i), Print(c)).map(|_| j += 1)
            })?;
            i += 1;
        }
        i += 1;
        queue!(
            self.stdout,
            MoveTo(0, i),
            Print("hjkl - move; c - clear; r - resize; x - solve; space - toggle queen; q - quit")
        )?;
        self.messages.iter().try_for_each(|m| {
            i += 1;
            queue!(self.stdout, MoveTo(0, i), Print(m))
        })?;
        queue!(self.stdout, MoveTo(self.pos.0, self.pos.1))?;
        self.stdout.flush()
    }
}

fn main() -> io::Result<()> {
    let mut state = State::new(8);

    // initialize the ui
    enable_raw_mode()?;
    execute!(
        state.stdout,
        terminal::EnterAlternateScreen,
        cursor::MoveTo(0, 0),
        cursor::Show,
    )?;

    state.render()?;
    while state.input()? {
        state.render()?;
    }

    // drop the ui
    disable_raw_mode()?;
    state.stdout.flush()?;
    execute!(
        state.stdout,
        terminal::Clear(terminal::ClearType::Purge),
        terminal::LeaveAlternateScreen
    )?;
    println!("bye!");

    Ok(())
}

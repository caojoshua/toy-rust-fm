use clifs::{get_ui_closure, ls_cwd, State};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, process::Command, time::Duration};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = State::new();
    terminal.draw(get_ui_closure(&mut state))?;

    loop {
        if let Ok(true) = event::poll(Duration::from_secs(1)) {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    event::KeyCode::Char('q') => {
                        // restore terminal
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;

                        return Ok(());
                    }
                    event::KeyCode::Char('j') => {
                        if state.selected < state.dir_entries.len() - 1 {
                            state.selected += 1;
                        } else {
                            state.selected = 0;
                        }
                        terminal.draw(get_ui_closure(&mut state))?;
                    }
                    event::KeyCode::Char('k') => {
                        if state.selected == 0 {
                            state.selected = state.dir_entries.len() - 1;
                        } else {
                            state.selected -= 1;
                        }
                        terminal.draw(get_ui_closure(&mut state))?;
                    }
                    event::KeyCode::Char('h') => {
                        state.cwd.pop();
                        state.dir_entries.clear();
                        state.dir_entries = ls_cwd(&state);
                        terminal.draw(get_ui_closure(&mut state))?;
                    }
                    event::KeyCode::Char('l') => {
                        let entry = state.dir_entries[state.selected].as_path();
                        if entry.is_dir() {
                            state.cwd = entry.to_path_buf();
                            state.selected = 0;
                            state.dir_entries.clear();
                            state.dir_entries = ls_cwd(&state);
                            terminal.draw(get_ui_closure(&mut state))?;
                        } else {
                            Command::new("nano").args([entry.to_str().unwrap()]).status()?;
                            terminal.clear()?;
                            terminal.draw(get_ui_closure(&mut state))?;
                        }
                    }
                    event::KeyCode::Char('b') => {
                        terminal.clear()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            )?;
                        disable_raw_mode()?;
                        Command::new("bash").status()?;
                        enable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            EnterAlternateScreen,
                            )?;
                        terminal.clear()?;
                        terminal.draw(get_ui_closure(&mut state))?;
                    }
                    _ => (),
                }
            }
        }
    }
}

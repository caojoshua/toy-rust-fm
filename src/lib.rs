use std::env::current_dir;
use std::fs::read_dir;
use std::path::PathBuf;
use std::vec;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
    Frame,
};

pub struct State {
    pub cwd: PathBuf,
    pub dir_entries: Vec<PathBuf>,
    pub selected: usize,
}

impl State {
    pub fn new() -> State {
        let cwd = current_dir().unwrap();
        let entries = ls(&cwd);
        State {
            cwd,
            dir_entries: entries,
            selected: 0,
        }
    }
}

pub fn ls_cwd(state: &State) -> Vec<PathBuf> {
    ls(&state.cwd)
}

pub fn ls(path: &PathBuf) -> Vec<PathBuf> {
    let mut entries = vec![];
    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        entries.push(entry.path());
    }
    entries
}

pub fn get_ui_closure<B>(state: &State) -> Box<dyn FnOnce(&mut Frame<B>) + '_>
where
    B: Backend,
{
    let ui = |f: &mut Frame<B>| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let mut items = vec![];
        for entry in &state.dir_entries {
            items.push(ListItem::new(
                entry.as_path().file_name().unwrap().to_str().unwrap(),
            ));
        }

        let list = List::new(items).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
        let mut list_state = ListState::default();
        list_state.select(Some(state.selected));
        f.render_stateful_widget(list, chunks[0], &mut list_state);
    };

    Box::new(ui)
}

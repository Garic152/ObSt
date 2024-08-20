use std::{self, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Alignment,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame, Terminal,
};

enum AppState {
    StartMenu,
    NewObservation,
    AddObservation,
}

pub struct App {
    state: AppState,
    quit: bool,
}

impl App {
    pub fn default() -> Self {
        App {
            state: AppState::StartMenu,
            quit: false,
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        if app.quit {
            return Ok(());
        }
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::StartMenu => match key.code {
                    KeyCode::Char('q') => app.quit = true,
                    KeyCode::Char('1') => {
                        app.state = AppState::NewObservation;
                    }
                    KeyCode::Char('2') => {
                        app.state = AppState::AddObservation;
                    }
                    _ => {}
                },
                AppState::NewObservation => match key.code {
                    KeyCode::Char('q') => app.state = AppState::StartMenu,
                    _ => {}
                },
                AppState::AddObservation => match key.code {
                    KeyCode::Char('q') => app.state = AppState::StartMenu,
                    _ => {}
                },
            }
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    match app.state {
        AppState::StartMenu => {
            let title = Title::from(" ObSt Main Menu ".bold());

            frame.render_widget(
                Paragraph::new("\nWelcome to the ObSt Observation tracking tool!\n")
                    .centered()
                    .block(Block::bordered().title(title.alignment(Alignment::Center)))
                    .alignment(Alignment::Center),
                frame.area(),
            );
        }
        AppState::NewObservation => {
            let title = Title::from(" New Observation ".bold());

            frame.render_widget(
                Paragraph::new("\nTest")
                    .centered()
                    .block(Block::bordered().title(title.alignment(Alignment::Center)))
                    .alignment(Alignment::Center),
                frame.area(),
            );
        }
        AppState::AddObservation => {
            let title = Title::from(" Add Observation ".bold());

            frame.render_widget(
                Paragraph::new("\nTest")
                    .centered()
                    .block(Block::bordered().title(title.alignment(Alignment::Center)))
                    .alignment(Alignment::Center),
                frame.area(),
            );
        }
    }
}

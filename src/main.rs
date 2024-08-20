use std::{self, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Block, Paragraph},
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

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
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
    let res = run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.area(),
    );
}

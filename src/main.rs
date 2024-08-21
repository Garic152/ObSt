use std::{self, io, str};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
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

enum NewObservationSteps {
    Name,
    Date,
    Amount,
    Declaration,
    Confirmation,
}

pub struct App {
    state: AppState,
    selected_menu: i8,
    new_observation_step: NewObservationSteps,
    confirmations_left: i8,
    observation: Observation,
    input: String,
    character_index: usize,
    data: Vec<String>,
    quit: bool,
}

struct Observation {
    name: String,
    parameters: Vec<Vec<String>>,
}

impl App {
    pub fn default() -> Self {
        App {
            state: AppState::StartMenu,
            selected_menu: 0,
            new_observation_step: NewObservationSteps::Name,
            confirmations_left: 0,
            observation: Observation {
                name: ("".to_string()),
                parameters: (Vec::new()),
            },
            input: String::new(),
            character_index: 0,
            data: Vec::new(),
            quit: false,
        }
    }
    pub fn new_observation_process_confirmation(&mut self) {
        match self.new_observation_step {
            NewObservationSteps::Date => {}
            _ => {}
        }
    }
    pub fn new_observation_add_date(&mut self) {
        self.observation
            .parameters
            .push(vec![String::from("Date"), String::from("DATE")]);
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
                        app.confirmations_left = 1;
                    }
                    KeyCode::Char('2') => {
                        app.state = AppState::AddObservation;
                    }
                    _ => {}
                },
                AppState::NewObservation => match key.code {
                    KeyCode::Char('q') => app.state = AppState::StartMenu,

                    _ => match app.new_observation_step {
                        NewObservationSteps::Date => match key.code {
                            KeyCode::Char('y') => app.new_observation_add_date(),
                            KeyCode::Char('n') => {
                                app.new_observation_step = NewObservationSteps::Amount
                            }
                            _ => {}
                        },
                        _ => {}
                    },
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
                Paragraph::new(
                    "\nWelcome to the ObSt Observation tracking tool!\n
                    \nPress one of the following keys to enter the corresponding submenus:
                    \n1) Create a new observation
                    \n2) Add to an existing observation
                    \nq) Quit the TUI",
                )
                .centered()
                .block(Block::bordered().title(title.alignment(Alignment::Center)))
                .alignment(Alignment::Center),
                frame.area(),
            );
        }
        AppState::NewObservation => {
            let title = Title::from(" New Observation ".bold());

            let outer_block = Block::bordered().title(title);
            let message = match app.new_observation_step {
                NewObservationSteps::Name => Paragraph::new("\nYou will now be able to add a new observation to the database.\nStarting off, give your observation a name.").centered(),
                NewObservationSteps::Date => Paragraph::new("\nDo you intend to track the date for your observation?").centered(),
                NewObservationSteps::Amount => Paragraph::new("\nHow many correlated variables do you want to observe?").centered(),
                NewObservationSteps::Declaration => Paragraph::new("\nPlease now declare the variables you want to observe.\nIn order to do that, fill in the highlighted box and press enter to confirm.").centered(),
                NewObservationSteps::Confirmation => Paragraph::new("\nTODO!")
            };

            let inner_block = Block::bordered()
                .title("Input")
                .title_alignment(Alignment::Center);
            let input = match app.new_observation_step {
                NewObservationSteps::Name => {
                    Paragraph::new(app.input.as_str()).style(Style::default())
                }
                _ => Paragraph::new("\nTODO!"),
            };

            let inner_area = outer_block.inner(frame.area());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(5), Constraint::Length(3)].as_ref())
                .split(inner_area);

            frame.render_widget(
                message
                    .centered()
                    .block(Block::default())
                    .alignment(Alignment::Center),
                chunks[0],
            );
            frame.render_widget(input.block(inner_block), chunks[1]);
            frame.render_widget(outer_block, frame.area());
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

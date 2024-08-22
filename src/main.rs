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

enum InputType {
    Name,
    Type,
}

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
    observation: Observation,
    amount: i8,
    input: String,
    input_type: InputType,
    input_vector: Vec<String>,
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
            observation: Observation {
                name: ("".to_string()),
                parameters: (Vec::new()),
            },
            amount: 0,
            input: "_".to_string(),
            input_type: InputType::Name,
            input_vector: Vec::new(),
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
    pub fn new_observation_add_name(&mut self) {
        self.input.pop();
        if self.input.is_empty() {
            self.reset();
        }
        self.observation.name = self.input.clone();
        self.new_observation_step = NewObservationSteps::Date;
        self.reset_string();
    }
    pub fn new_observation_add_date(&mut self) {
        self.observation
            .parameters
            .push(vec![String::from("Date"), String::from("DATE")]);
        self.new_observation_step = NewObservationSteps::Amount;
    }
    pub fn new_observation_add_amount(&mut self) {
        self.input.pop();
        self.amount = match self.input.trim().parse::<i8>() {
            Ok(num) => num,
            _ => {
                self.reset();
                0
            }
        };
        self.new_observation_step = NewObservationSteps::Declaration;
        self.reset_string();
    }
    pub fn append_input_vector(&mut self) {
        self.input.pop();
        self.input_vector.push(self.input.clone());
        match self.input_type {
            InputType::Type => self.reset_string(),
            _ => {}
        }
        self.amount -= 1;
        if self.amount <= 0 {
            self.new_observation_step = NewObservationSteps::Confirmation;
        }
        self.reset_string();
    }
    pub fn reset_string(&mut self) {
        self.input = "_".to_string();
    }
    pub fn reset(&mut self) {
        *self = App::default();
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
                    KeyCode::Char('q') => app.reset(),

                    _ => match app.new_observation_step {
                        NewObservationSteps::Name => match key.code {
                            KeyCode::Char(' ') => {}
                            KeyCode::Enter => {
                                app.new_observation_add_name();
                            }
                            KeyCode::Backspace => {
                                app.input.pop();
                                app.input.pop();
                                app.input.push('_');
                            }
                            KeyCode::Char(to_insert) => {
                                app.input.pop();
                                app.input.push(to_insert);
                                app.input.push('_');
                            }
                            _ => {}
                        },
                        NewObservationSteps::Date => match key.code {
                            KeyCode::Char('y') => app.new_observation_add_date(),
                            KeyCode::Enter => app.new_observation_add_date(),
                            KeyCode::Char('n') => {
                                app.new_observation_step = NewObservationSteps::Amount
                            }
                            _ => {}
                        },
                        NewObservationSteps::Amount => match key.code {
                            KeyCode::Char(' ') => {}
                            KeyCode::Enter => app.new_observation_add_amount(),
                            KeyCode::Backspace => {
                                app.input.pop();
                                app.input.pop();
                                app.input.push('_');
                            }
                            KeyCode::Char(to_insert) => {
                                app.input.pop();
                                app.input.push(to_insert);
                                app.input.push('_');
                            }
                            _ => {}
                        },
                        NewObservationSteps::Declaration => match key.code {
                            KeyCode::Char(' ') => {}
                            KeyCode::Enter => match app.input_type {
                                InputType::Name => app.append_input_vector(),
                                InputType::Type => app.append_input_vector(),
                            },
                            KeyCode::Backspace => {
                                app.input.pop();
                                app.input.pop();
                                app.input.push('_');
                            }
                            KeyCode::Char(to_insert) => {
                                app.input.pop();
                                app.input.push(to_insert);
                                app.input.push('_');
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
                NewObservationSteps::Name => Paragraph::new("\nYou will now be able to add a new observation to the database.\nStarting off, give your observation a name."),
                NewObservationSteps::Date => Paragraph::new("\nDo you intend to track the date for your observation? (Y/n)"),
                NewObservationSteps::Amount => Paragraph::new("\nHow many correlated variables do you want to observe?"),
                NewObservationSteps::Declaration => Paragraph::new("\nPlease now declare the variables you want to observe.\nIn order to do that, fill in the highlighted box and press enter to confirm."),
                NewObservationSteps::Confirmation => Paragraph::new("\nTODO!")
            };

            let inner_area = outer_block.inner(frame.area());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(4), Constraint::Length(3)].as_ref())
                .split(inner_area);

            frame.render_widget(
                message
                    .centered()
                    .block(Block::default())
                    .alignment(Alignment::Left),
                chunks[0],
            );

            match app.new_observation_step {
                NewObservationSteps::Date => {}
                _ => {
                    let inner_block = Block::bordered()
                        .title("Input")
                        .title_alignment(Alignment::Left);
                    let input = Paragraph::new(app.input.as_str()).style(Style::default());
                    frame.render_widget(input.block(inner_block), chunks[1]);
                }
            }

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

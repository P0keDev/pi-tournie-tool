use crate::{
    event::{AppEvent, Event, EventHandler},
    gpio::Button,
    tabs::{ExitTab, ReplaysTab, SdTab, SmashscopeTab, TabWidget},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Menu,
    Tab,
    Exiting,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub screen: CurrentScreen,

    pub tabs: Vec<Box<dyn TabWidget>>,
    pub tab_index: usize,

    /// Event handler.
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            screen: CurrentScreen::Menu,
            tabs: vec![
                Box::new(SdTab::default()),
                Box::new(ReplaysTab::default()),
                Box::new(SmashscopeTab::default()),
                Box::new(ExitTab::default()),
            ],
            tab_index: 0,
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            },
            Event::Gpio(button) => self.handle_gpio_event(button)?,

            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // Emulates GPIO inputs with asdf keys
            KeyCode::Char('a') => self.events.send_gpio(Button::Up),
            KeyCode::Char('s') => self.events.send_gpio(Button::Down),
            KeyCode::Char('d') => self.events.send_gpio(Button::Select),
            KeyCode::Char('f') => self.events.send_gpio(Button::Back),

            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the GPIO events and updates the state of [`App`].
    pub fn handle_gpio_event(&mut self, button: Button) -> color_eyre::Result<()> {
        let current_tab = &mut self.tabs[self.tab_index];

        match self.screen {
            CurrentScreen::Menu => match button {
                Button::Up => self.prev_tab(),
                Button::Down => self.next_tab(),
                Button::Select => {
                    self.screen = CurrentScreen::Tab;
                    current_tab.open();
                }
                _ => {}
            },
            CurrentScreen::Tab => match button {
                Button::Back => {
                    self.screen = CurrentScreen::Menu;
                    current_tab.close();
                }
                _ => {
                    current_tab.handle_gpio_event(button, &mut self.events);
                }
            },
            _ => {}
        }

        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        if self.tab_index + 1 < self.tabs.len() {
            self.tab_index += 1;
        }
    }

    pub fn prev_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        }
    }

    pub fn current_tab_mut(&mut self) -> &mut Box<dyn TabWidget> {
        &mut self.tabs[self.tab_index]
    }

    pub fn current_tab(&self) -> &Box<dyn TabWidget> {
        &self.tabs[self.tab_index]
    }
}

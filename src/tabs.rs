use std::{
    io,
    process::{self, Child, Stdio},
};

use ratatui::{
    layout::Rect, style::{palette::tailwind, Stylize}, text::Line, widgets::Paragraph, Frame
};

use crate::{disk::SDHandler, event::{AppEvent, EventHandler}, gpio::Button, ui};

#[derive(Default)]
pub struct TabState {
    pub active: bool,
}

pub trait TabWidget {
    fn tab_state(&self) -> &TabState;
    fn tab_state_mut(&mut self) -> &mut TabState;

    fn tab_name(&self) -> String;
    fn tab_color(&self) -> tailwind::Palette;

    fn open(&mut self) {
        self.tab_state_mut().active = true;
    }

    fn close(&mut self) {
        self.tab_state_mut().active = false;
    }

    fn handle_gpio_event(&mut self, _button: Button, _events: &mut EventHandler) {}

    fn render(&self, frame: &mut Frame, area: Rect);
}

impl core::fmt::Debug for dyn TabWidget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TabWidget{{{}}}", self.tab_name())
    }
}

#[derive(Default)]
pub struct SdTab {
    pub state: TabState,
    pub sd_card: Option<SDHandler>,
    pub version: Option<String>,
}

impl TabWidget for SdTab {
    fn tab_state(&self) -> &TabState {
        &self.state
    }

    fn tab_state_mut(&mut self) -> &mut TabState {
        &mut self.state
    }

    fn tab_name(&self) -> String {
        String::from("SD Card")
    }

    fn tab_color(&self) -> tailwind::Palette {
        tailwind::GREEN
    }

    fn open(&mut self) {
        if self.state.active {
            return;
        }
        self.state.active = true;

        self.sd_card = SDHandler::find_sd();
        self.version = None;
        if let Some(h) = &self.sd_card {
            self.version = h.get_slippi_version();
        }
    }

    fn close(&mut self) {
        self.state.active = false;
        self.sd_card = None;
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        if !self.state.active { return; }

        let area = ui::centered_rect(area, 8);

        let mut lines: Vec<Line> = Vec::new();
        if let Some(handler) = &self.sd_card {
            lines.push(Line::raw("Found SD Card!"));

            if let Some(version) = &self.version {
                lines.push(Line::raw(format!("Slippi Version: {}", version)));
            } else {
                lines.push(Line::raw("No Slippi Nintendont Installation!"));
            }
        } else {
            lines.push(Line::raw("No SD Card!"));
        }

        let paragraph = Paragraph::new(lines)
            .fg(tailwind::SLATE.c200)
            .centered();

        frame.render_widget(paragraph, area);
    }
}

#[derive(Default)]
pub struct ReplaysTab {
    pub state: TabState,
}

impl TabWidget for ReplaysTab {
    fn tab_state(&self) -> &TabState {
        &self.state
    }

    fn tab_state_mut(&mut self) -> &mut TabState {
        &mut self.state
    }

    fn tab_name(&self) -> String {
        String::from("Replays")
    }

    fn tab_color(&self) -> tailwind::Palette {
        tailwind::FUCHSIA
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        // TODO
    }
}

#[derive(Default)]
pub struct SmashscopeTab {
    pub state: TabState,
    pub dolphin: Option<Child>,
}

impl TabWidget for SmashscopeTab {
    fn tab_state(&self) -> &TabState {
        &self.state
    }

    fn tab_state_mut(&mut self) -> &mut TabState {
        &mut self.state
    }

    fn tab_name(&self) -> String {
        String::from("Smashscope")
    }

    fn tab_color(&self) -> tailwind::Palette {
        tailwind::INDIGO
    }

    fn open(&mut self) {
        if self.state.active {
            return;
        }
        self.state.active = true;

        let result = process::Command::new("dolphin-emu")
            .args(["-b", "-e", "boot.dol"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        if let io::Result::Ok(c) = result {
            self.dolphin = Some(c);
        }
    }

    fn close(&mut self) {
        self.state.active = false;

        if let Some(mut c) = self.dolphin.take() {
            let _ = c.kill();
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let area = ui::centered_rect(area, 8);
        let mut lines = vec![Line::raw("Press [OK] to launch Dolphin")];
        if self.state.active {
            lines.append(&mut vec![
                Line::raw(""),
                Line::raw("Launching Dolphin..."),
                Line::raw("Press [BACK] to quit"),
            ]);
        }

        let paragraph = Paragraph::new(lines)
            .fg(tailwind::SLATE.c200)
            .centered();

        frame.render_widget(paragraph, area);
    }
}

#[derive(Default)]
pub struct ExitTab {
    pub state: TabState,
}

impl TabWidget for ExitTab {
    fn tab_state(&self) -> &TabState {
        &self.state
    }

    fn tab_state_mut(&mut self) -> &mut TabState {
        &mut self.state
    }

    fn tab_name(&self) -> String {
        String::from("Exit")
    }

    fn tab_color(&self) -> tailwind::Palette {
        tailwind::RED
    }

    fn handle_gpio_event(&mut self, button: Button, events: &mut EventHandler) {
        match button {
            Button::Select => events.send(AppEvent::Quit),
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let area = ui::centered_rect(area, 8);
        let mut lines = vec![Line::raw("Press [OK] to exit")];
        if self.state.active {
            lines.push(Line::raw("Press [OK] again to confirm"));
        }

        let paragraph = Paragraph::new(lines)
            .fg(tailwind::SLATE.c200)
            .centered();

        frame.render_widget(paragraph, area);
    }
}
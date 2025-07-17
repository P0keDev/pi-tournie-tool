use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect}, style::{palette::tailwind, Style, Stylize}, symbols, text::Line, widgets::{Block, Padding, Tabs}, Frame
};

use crate::{app::App, tabs::TabWidget, VERSION};

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header, inner, footer] = vertical.areas(frame.area());

        let horizontal = Layout::horizontal([Min(0), Length(10)]);
        let [tab_headers, title] = horizontal.areas(header);

        render_background(frame, frame.area());
        render_title(frame, title);
        self.render_tab_headers(frame, tab_headers);
        render_footer(frame, footer);

        self.render_tab(frame, inner);
    }

    fn render_tab_headers(&self, frame: &mut Frame, area: Rect) {
        let titles = self.tabs.iter().map(tab_title);
        let highlight_style = Style::new().bg(self.current_tab().tab_color().c700);

        let widget = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(self.tab_index)
            .padding("", "")
            .divider(" ");
        frame.render_widget(widget, area);
    }

    fn render_tab(&self, frame: &mut Frame, area: Rect) {
        // render tab window outline, which is colored if the current tab is active
        frame.render_widget(tab_block(self.current_tab()), area);

        // call tab's render function
        self.current_tab().render(frame, area.inner(Margin::new(1, 1)));
    }
}

fn render_background(frame: &mut Frame, area: Rect) {
    frame.render_widget(Block::new().bg(tailwind::SLATE.c950), area);
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(format!("TT v{}", VERSION).bold().into_right_aligned_line().fg(tailwind::SLATE.c200), area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let line = Line::raw("   left   |    right    |    OK     |   back   ")
        .bg(tailwind::NEUTRAL.c900)
        .fg(tailwind::SLATE.c200)
        .centered();
    frame.render_widget(line, area);
}

fn tab_title(tab: &Box<dyn TabWidget>) -> Line {
    format!(" {} ", tab.tab_name())
        .fg(tailwind::SLATE.c200)
        //.bg(tab.tab_color().c900)
        .bg(tailwind::NEUTRAL.c700)
        .into()
}

fn tab_block(tab: &Box<dyn TabWidget>) -> Block {
    let color = if tab.tab_state().active { tab.tab_color().c700 } else { tailwind::NEUTRAL.c500 };
    Block::bordered()
        .border_set(symbols::border::PROPORTIONAL_TALL)
        .padding(Padding::horizontal(1))
        .border_style(color)
}

pub fn centered_rect(area: Rect, size: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(size)])
        .flex(Flex::Center)
        .areas(area);
    area
}
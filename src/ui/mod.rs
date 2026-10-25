use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::app::{App, State};

pub fn render(app: &App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .margin(1)
        .split(frame.area());

    render_question(app, frame, layout[0]);
    render_textarea(app, frame, layout[1]);
    render_answer(app, frame, layout[2]);
}

fn render_question(app: &App, frame: &mut Frame, area: Rect) {
    let text = Text::from(app.current_card().explanation.as_str());
    let paragraph = Paragraph::new(text);

    frame.render_widget(paragraph, area);
}

fn render_textarea(app: &App, frame: &mut Frame, area: Rect) {
    let style = match app.state {
        State::Input => Style::new(),
        State::Review(success) => {
            if success {
                Style::new().green()
            } else {
                Style::new().red()
            }
        }
        State::ForceType => Style::new().yellow(),
    };

    let block = Block::bordered().title("Odpowiedź").style(style);

    let text = Text::from(app.input.as_str());
    let paragraph = Paragraph::new(text).block(block);

    frame.render_widget(paragraph, area);
}

fn render_answer(app: &App, frame: &mut Frame, area: Rect) {
    if matches!(app.state, State::Review(false) | State::ForceType) {
        let text = Text::from(app.current_card().definition.as_str());
        let paragraph = Paragraph::new(text);

        frame.render_widget(paragraph, area);
    }
}
